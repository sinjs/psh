use std::process::exit as process_exit;

pub fn exit(args: &Vec<String>) -> Result<i32, i32> {
    if args.len() == 1 {
        process_exit(0);
    } else {
        process_exit(args[1].parse().unwrap_or(1));
    }
}

pub fn version() -> Result<i32, i32> {
    println!("{} v{}-{}", super::NAME, super::VERSION, super::TARGET);
    Ok(0)
}
