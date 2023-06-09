mod commands;
mod config;
mod constants;
mod macros;

#[cfg(test)]
mod test;

use colored::*;
use config::create_config_object;
use config::ConfigManager;
use std::io;
use std::io::ErrorKind;
use std::process;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: ConfigManager = create_config_object();
}

fn main() {
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    if !CONFIG.data.get_no_greeting() {
        println!(
            "welcome to {} v{}-{}",
            constants::NAME,
            constants::VERSION,
            constants::TARGET
        );
    }

    cdbg(&CONFIG.data);

    let mut has_failed = false;

    loop {
        match interpreter(has_failed) {
            Ok(_) => has_failed = false,
            Err(_) => has_failed = true,
        }
    }
}

#[derive(Debug)]
enum CommandExecutionError<T> {
    NotFound,
    ExitCode(T),
}

fn cdbg(msg: impl std::fmt::Debug) {
    if CONFIG.data.get_debug() {
        dbg!(msg);
    }
}

fn parse_command(input: String) -> Option<String> {
    let trimmed = input.trim().to_string();

    match trimmed.len() {
        0 => None,
        _ => Some(trimmed),
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(err),
    }
}

fn parse_args(command: String) -> Vec<String> {
    command.split_whitespace().map(|s| s.to_string()).collect()
}

fn try_execute_binary(args: &Vec<String>) -> Result<(), CommandExecutionError<i32>> {
    let mut command = process::Command::new(&args[0]);
    command.args(args.split_first().unwrap().1);

    match command.spawn() {
        Ok(mut c) => {
            c.wait().unwrap();
            Ok(())
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(CommandExecutionError::NotFound),
            _ => panic!("Failed to execute command: {:?}", err),
        },
    }
}

fn execute_command(args: &Vec<String>) -> Result<(), CommandExecutionError<i32>> {
    let cmd = &args[0];

    match cmd.as_str() {
        "exit" => match commands::exit(args) {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        "version" => match commands::version() {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        "help" => match commands::help() {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        "ls" => match commands::ls() {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        "cd" => match commands::cd(args) {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        "pwd" => match commands::pwd() {
            Ok(_) => Ok(()),
            Err(exit_code) => Err(CommandExecutionError::ExitCode(exit_code)),
        },
        _ => try_execute_binary(&args),
    }
}

fn interpreter(has_failed: bool) -> Result<(), ()> {
    // TODO: support config(custom-prompt)
    flushprint!(
        "{} ",
        (|| {
            match has_failed {
                true => "$".bright_red(),
                false => "$".into(),
            }
        })()
    );

    let command = parse_command(read_line().unwrap());

    let command = match command {
        None => return Ok(()),
        Some(cmd) => cmd,
    };

    let args = parse_args(command.clone());
    let execute_result = execute_command(&args);

    match execute_result {
        Ok(()) => Ok(()),
        Err(err) => match err {
            CommandExecutionError::NotFound => {
                eprintln!("{}: {}: command not found", constants::NAME, &args[0]);
                Err(())
            }
            CommandExecutionError::ExitCode(code) => {
                eprintln!(
                    "{}: {}: command exited with code {}",
                    constants::NAME,
                    &args[0],
                    code
                );
                Err(())
            }
        },
    }
}
