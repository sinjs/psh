use super::*;

#[test]
fn macro_flushprint() {
    flushprint!("Hello, world!");
}

#[test]
fn macro_flushprint_fmt() {
    flushprint!("Hello, {}! Number: {}", "world", 26);
}

#[test]
fn macro_flushprint_utf8() {
    flushprint!("😀ΣςσΦφΩἉᾺἁὰᾸᾰἋ");
}

mod test_commands {
    use super::*;

    #[test]
    fn help() {
        commands::help().unwrap();
    }

    #[test]
    fn version() {
        commands::version().unwrap();
    }

    #[test]
    fn ls() {
        commands::ls().unwrap();
    }

    #[test]
    fn cd_valid_path() {
        commands::cd(&vec!["cd".to_string(), "/".to_string()]).unwrap();
    }

    #[test]
    fn cd_invalid_path() {
        commands::cd(&vec!["cd".to_string(), "\\\\".to_string()]).unwrap_err();
    }

    #[test]
    fn pwd() {
        commands::pwd().unwrap();
    }
}

mod test_config {
    use crate::config::{self, ConfigData};

    #[test]
    fn create_config() {
        let obj = config::create_config_object();
        assert_eq!(obj.data.get_debug(), false);
        assert_eq!(obj.data.get_no_greeting(), false);
    }

    #[test]
    fn config_data() {
        let mut obj = config::ConfigManager::new(ConfigData {
            debug: true,
            no_greeting: false,
        });

        assert_eq!(obj.data.get_debug(), true);
        assert_eq!(obj.data.get_no_greeting(), false);

        obj.data.set_debug(false);
        obj.data.set_no_greeting(true);

        assert_eq!(obj.data.get_debug(), false);
        assert_eq!(obj.data.get_no_greeting(), true);
    }
}
