use std::env;

pub fn create_config_object() -> ConfigManager {
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

#[derive(Debug)]
pub struct ConfigData {
    debug: bool,
    no_greeting: bool,
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
