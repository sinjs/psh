mod commands;
mod config;
mod macros;

use config::create_config_object;
use config::ConfigManager;
use std::env;
use std::io;
use std::io::ErrorKind;
use std::process;

#[macro_use]
extern crate lazy_static;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
const TARGET: &str = "dev";
#[cfg(not(debug_assertions))]
const TARGET: &str = "release";

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: ConfigManager = create_config_object();
}

fn main() {
    if !CONFIG.data.get_no_greeting() {
        println!("welcome to {} v{}-{}", NAME, VERSION, TARGET);
    }

    dbg!(&CONFIG.data);

    loop {
        interpreter();
    }
}

#[derive(Debug)]
enum CommandExecutionError<T> {
    NotFound,
    ExitCode(T),
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
        _ => try_execute_binary(&args),
    }
}

fn interpreter() -> () {
    // TODO: support config(custom-prompt)
    flushprint!("{} ", "$");

    let command = parse_command(read_line().unwrap());

    let command = match command {
        None => return,
        Some(cmd) => cmd,
    };

    let args = parse_args(command.clone());
    let execute_result = execute_command(&args);

    match execute_result {
        Ok(()) => (),
        Err(err) => {
            match err {
                CommandExecutionError::NotFound => {
                    println!("{}: {}: command not found", NAME, &args[0]);
                }
                CommandExecutionError::ExitCode(code) => {
                    println!("{}: {}: command exited with code {}", NAME, &args[0], code);
                }
            };
        }
    };

    dbg!(command, args);
}
