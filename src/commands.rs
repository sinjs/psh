use std::env;
use std::fs;
use std::process::exit as process_exit;

use crate::constants;
use crate::flushprint;

pub fn exit(args: &Vec<String>) -> Result<i32, i32> {
    if args.len() == 1 {
        process_exit(0);
    } else {
        process_exit(args[1].parse().unwrap_or(1));
    }
}

pub fn version() -> Result<i32, i32> {
    println!(
        "{} v{}-{}",
        constants::NAME,
        constants::VERSION,
        constants::TARGET
    );
    Ok(0)
}

pub fn help() -> Result<i32, i32> {
    const HELP_TEXT: &str = include_str!("../data/help.txt");
    flushprint!("{}", HELP_TEXT);
    Ok(0)
}

pub fn ls() -> Result<i32, i32> {
    let cwd = env::current_dir().unwrap();
    let contents = fs::read_dir(&cwd).unwrap();

    for content in contents {
        print!("{} ", content.unwrap().file_name().into_string().unwrap());
    }

    flushprint!("\n");

    Ok(0)
}

pub fn cd(args: &Vec<String>) -> Result<i32, i32> {
    if args.len() <= 1 {
        println!("no arguments specified (see help)");
        return Err(1);
    }

    let dir = args.get(1).unwrap();

    match env::set_current_dir(dir) {
        Ok(_) => Ok(0),
        Err(err) => {
            eprintln!("psh: cd: {}: {}", dir, err.to_string());
            Err(1)
        }
    }
}

pub fn pwd() -> Result<i32, i32> {
    println!("{}", env::current_dir().unwrap().display());
    Ok(0)
}
