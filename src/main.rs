mod commands;
mod macros;
use std::env;
use std::io;
use std::io::ErrorKind;
use std::process;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
const TARGET: &str = "dev";
#[cfg(not(debug_assertions))]
const TARGET: &str = "release";

struct Config {
    pub no_greeting: bool,
}

#[derive(Debug)]
enum CommandExecutionError<T> {
    NotFound,
    ExitCode(T),
}

fn parse_command(input: String) -> Option<String> {
    let trimmed = input.trim().to_string();
    if trimmed.len() == 0 {
        return None;
    }
    Some(trimmed)
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

fn parse_shell_args() -> Result<Vec<String>, ()> {
    let mut shell_args: Vec<String> = env::args().collect();
    shell_args.reverse();
    shell_args.pop();
    shell_args.reverse();
    Ok(shell_args)
}

fn try_execute_binary(args: &Vec<String>) -> Result<(), CommandExecutionError<i32>> {
    let mut child = process::Command::new(&args[0]);
    child.args(args.split_first().unwrap().1);

    let spawned = child.spawn();

    match spawned {
        Ok(_) => Ok(()),
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

fn main() {
    let mut shell_args = parse_shell_args().expect("Failed to parse arguments");
    let mut shell_args_temp = shell_args.clone();
    let mut config = Config { no_greeting: false };

    for (index, argument) in shell_args.iter_mut().enumerate() {
        if argument == &String::from("--no-greeting") {
            config.no_greeting = true;
            // TODO: support config(custom-shell-greeting)
            //                 args(custom-shell-greeting)
            shell_args_temp.remove(index);
        }
    }

    if !config.no_greeting {
        println!("welcome to {} v{}-{}", NAME, VERSION, TARGET);
    }

    let shell_args = shell_args_temp;

    dbg!(shell_args);

    loop {
        interpreter();
    }
}
