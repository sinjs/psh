pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
pub const TARGET: &str = "dev";
#[cfg(not(debug_assertions))]
pub const TARGET: &str = "release";

// psh configuration locations, specified in the order that they are prioritized
// warning: platform specific, please modify when porting to other platforms
#[cfg(target_os = "windows")]
pub const CONFIG_LOCATIONS: &[&str] = &[
    "C:\\.pshrc",
    "%USERPROFILE%\\.pshrc",
    "%USERPROFILE%\\.config\\.pshrc",
];

#[cfg(unix)]
pub const CONFIG_LOCATIONS: &[&str] = &["/.pshrc", "/etc/pshrc", "~/.pshrc", "~/.config/.pshrc"];

#[cfg(target_os = "pene")]
pub const CONFIG_LOCATIONS: &[&str] =
    &["/.pshrc", "/etc/pshrc", "~/.pshrc", "~/.config/PeneShellRC"];
