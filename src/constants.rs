pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
pub const TARGET: &str = "dev";
#[cfg(not(debug_assertions))]
pub const TARGET: &str = "release";
