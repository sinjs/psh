use crate::constants;
use colored::*;
use ini::Ini;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io;
use std::path::Path;

type FileConfig<'group_key> = HashMap<&'group_key str, HashMap<String, String>>;

pub fn create_config_object() -> ConfigManager {
    read_pshrc_to_map();
    let mut argv: Vec<String> = env::args().collect();
    let mut data = ConfigData {
        debug: false,
        no_greeting: false,
    };

    argv.remove(0);
    let argv = argv;

    for arg in argv.iter() {
        if arg == &String::from("--no-greeting") {
            data.set_no_greeting(true);
        }

        if arg == &String::from("--debug") {
            data.set_debug(true);
        }
    }

    ConfigManager { data }
}

#[cfg(windows)]
pub fn expand_env_vars(s: &str) -> std::io::Result<String> {
    lazy_static! {
        static ref ENV_VAR: Regex = Regex::new("%([[:word:]]*)%").expect("Regex is invalid");
    }

    let result: String = ENV_VAR
        .replace_all(s, |captures: &regex::Captures| match &captures[1] {
            "" => String::from("%"),
            varname => env::var(varname).expect("Environment variable is invalid"),
        })
        .into();

    Ok(result)
}

#[cfg(not(windows))]
pub fn expand_tilde(s: &str) -> String {
    shellexpand::tilde(s).into()
}

pub fn read_pshrc_to_map<'group_key>() -> FileConfig<'group_key> {
    let mut path: Option<String> = None;
    let mut map: FileConfig = HashMap::new();

    for location in constants::CONFIG_LOCATIONS {
        #[cfg(windows)]
        let location = expand_env_vars(location).unwrap();

        #[cfg(not(windows))]
        let location = expand_tilde(location);

        match Path::new(&location).try_exists() {
            Ok(exists) => match exists {
                true => {
                    path = Some(location.to_string());
                    break;
                }
                false => {
                    if cfg!(debug_assertions) {
                        dbg!(location);
                    }
                }
            },
            Err(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => continue,
                _ => {
                    eprintln!(
                        "pshrc: {} reading {}: {}",
                        "error:".bold().bright_red(),
                        location,
                        e
                    );
                }
            },
        }
    }

    if let Some(path) = path {
        let config_parsed = Ini::load_from_file(path).unwrap();
        for (selection, property) in config_parsed.iter() {
            match selection {
                Some(selection) => match selection.to_lowercase().as_str() {
                    "flags" => {
                        let mut property_map: HashMap<String, String> = HashMap::new();

                        for (key, value) in property.iter() {
                            property_map.insert(key.to_string(), value.to_string());
                        }

                        map.insert("flags", property_map);
                    }
                    _ => {
                        for (key, value) in property.iter() {
                            eprintln!(
                                "pshrc: {} ignoring property in invalid group {}: {}={}",
                                "warn:".bold().yellow(),
                                selection.bold(),
                                key,
                                value
                            );
                        }
                    }
                },
                None => {
                    for (key, value) in property.iter() {
                        eprintln!(
                            "pshrc: {} ignoring property without group: {}={}",
                            "warn:".bold().yellow(),
                            key,
                            value
                        );
                    }
                }
            }
        }
    };

    dbg!(&map);

    map
}

#[derive(Debug)]
pub struct ConfigData {
    pub debug: bool,
    pub no_greeting: bool,
}

impl ConfigData {
    pub fn get_debug(&self) -> bool {
        self.debug
    }

    pub fn set_debug(&mut self, value: bool) {
        self.debug = value
    }

    pub fn get_no_greeting(&self) -> bool {
        self.no_greeting
    }

    pub fn set_no_greeting(&mut self, value: bool) {
        self.no_greeting = value
    }
}

#[derive(Debug)]
pub struct ConfigManager {
    pub data: ConfigData,
}

impl ConfigManager {
    pub fn new(data: ConfigData) -> Self {
        ConfigManager { data }
    }
}
