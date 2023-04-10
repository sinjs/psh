#[macro_export]
macro_rules! flushprint {
    () => {
        use std::io::Write;
        std::print!("\n");
        std::io::stdout().flush().unwrap();
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        std::print!($($arg)*);
        std::io::stdout().flush().unwrap();
    }};
}
