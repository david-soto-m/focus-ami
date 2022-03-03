use crate::consts::{config, errors};
use crate::interact;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Config {
    interval: u8,
    stretch: u16,
    password: String,
    pub processes: HashSet<String>,
}

impl Config {
    /// Creates a new config from the parameters
    pub fn new(interval: u8, stretch: u16, password: &str, processes: Vec<String>) -> Self {
        Self {
            interval,
            stretch,
            password: password.to_string(),
            processes: HashSet::from_iter(processes),
        }
    }
    /// Get the config object from the config.yaml file.
    ///
    /// # Panics
    /// - if access to the file is compromised anywhere in it's path.
    /// Lack of access to the config directory is distinguished from lack of access to the file
    pub fn get_or_create(file: Option<String>) -> Config {
        let filename = match file {
            None => {
                let proj_dirs = ProjectDirs::from(
                    config::PROJECT_INFO.0,
                    config::PROJECT_INFO.1,
                    config::PROJECT_INFO.2,
                )
                .expect(errors::PROJECT);
                let proj_conf_dirs = proj_dirs.config_dir();
                if !proj_conf_dirs.exists() {
                    fs::create_dir_all(proj_conf_dirs).expect(errors::DIRECTORY);
                }
                proj_conf_dirs.join(config::FILENAME)
            }
            Some(file) => Path::new(".").join(file), // not clean but works, help here??
        };
        if !filename.exists() {
            Config::write_config(&Config::create(), &filename);
            // is this idiomatic??
        }
        serde_yaml::from_str::<Config>(&fs::read_to_string(&filename).expect(errors::R_FILE))
            .unwrap_or_else(|err| {
                println!("{}", err);
                let config = Config::create();
                Config::write_config(&config, &filename);
                config
            })
    }

    /// returns a Duration from the stretch. It transforms from minutes to seconds
    fn write_config(&self, filename: &Path) {
        fs::write(
            &(*filename),
            serde_yaml::to_string(&self).expect(errors::ENCODING),
        )
        .expect(errors::W_FILE);
    }
    pub fn get_stretch(&self) -> Duration {
        Duration::from_secs(self.stretch as u64 /* 60*/)
    }
    pub fn get_interval(&self) -> Duration {
        Duration::from_secs(self.interval as u64)
    }
    pub fn edit(&self) -> Config {
        Config::default()
    }

    /// Helps the user create a config, providing sane defaults and a sense
    pub fn create() -> Config {
        println!("We are going to create a config for you");
        println!("Whenever you are happy with the default type \\q");
        println!("The first item is the period after which to kill processes.");
        println!("Please provide a number that will be interpreted as seconds");
        println!("By default: 30 || Max: 255 || Min 1");
        let interval = match interact::utils::get_item() {
            Some(num) => {
                if num > 0 {
                    num
                } else {
                    1
                }
            }
            None => 30,
        };
        println!("Now we are going to determine the minutes you want to focus");
        println!("Keep in mind that you will have to wait for the whole period.");
        println!("Please provide a number that will be interpreted as minutes");
        println!("By default: 30 || Max: 65535");
        let stretch = interact::utils::get_item().unwrap_or(30);
        println!("The next step is setting up a password.");
        println!("This is not for security, but in order to be a nuisance");
        println!("That way you may be discouraged to quit the second you get distracted");
        println!("By default: a || Recommended : aslkdhgjhkadbchjqwmepam ionk");
        let passwd = match interact::utils::get_item() {
            Some(stri) => stri,
            None => "a".to_string(),
        };
        println!("Finally we are going to set up the processes to be blocked");
        println!("If you need help to get the name of a process please");
        println!("1. When you only have unknown processes type \\q");
        println!("2. Run \"focus -a\". Alternatively use ps -e (and diff?) to figure out the name of your process");
        println!("3. Add it to your config during a normal run");
        println!("To add a process type it and press enter");
        println!("To stop adding processes just type \\q");
        let processes = interact::utils::get_vec(interact::utils::get_item);
        Config::new(interval, stretch, &passwd, processes)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n---\nkill time: {} seconds\nwork time: {} minutes\npassword: {}\nprocesses: {:?}\n---\n",
            self.interval, self.stretch, self.password, self.processes
        )
    }
}
