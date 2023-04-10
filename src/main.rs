mod macros;
use std::env;
use std::io;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
const TARGET: &str = "dev";
#[cfg(not(debug_assertions))]
const TARGET: &str = "release";

struct Config {
    pub no_greeting: bool,
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

fn interpreter() -> () {
    // TODO: support config(custom-prompt)
    flushprint!("{} ", ">");

    let command = parse_command(read_line().unwrap());

    let command = match command {
        None => return,
        Some(cmd) => cmd,
    };

    let args = parse_args(command.clone());

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
