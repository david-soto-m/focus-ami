use crate::consts::{config, errors};
use crate::utils;
use crate::utils::GuessablePassword;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
mod interact;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Config {
    kill_time: u8,
    work_time: u16,
    password: String,
    pub processes: HashSet<String>,
}

impl Config {
    /// Creates a new config from the parameters
    pub fn new(kill_time: u8, work_time: u16, password: &str, processes: Vec<String>) -> Self {
        Self {
            kill_time,
            work_time,
            password: password.to_string(),
            processes: HashSet::from_iter(processes),
        }
    }
    fn new_internal(
        kill_time: u8,
        work_time: u16,
        password: String,
        processes: HashSet<String>,
    ) -> Self {
        Self {
            kill_time,
            work_time,
            password,
            processes,
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

    /// returns a Duration from the work_time. It transforms from minutes to seconds
    fn write_config(&self, filename: &Path) {
        fs::write(
            &(*filename),
            serde_yaml::to_string(&self).expect(errors::ENCODING),
        )
        .expect(errors::W_FILE);
    }
    pub fn get_work_time(&self) -> Duration {
        Duration::from_secs(self.work_time as u64 /* 60*/)
    }
    pub fn get_work_time_as_min(&self) -> u16 {
        self.work_time
    }
    pub fn get_kill_time(&self) -> Duration {
        Duration::from_secs(self.kill_time as u64)
    }
    pub fn get_kill_time_as_seconfs(&self) -> u8 {
        self.kill_time
    }
    pub fn print_curr_state(&self) {
        println!("Your config is :{}", self);
    }
    pub fn edit(mut self) -> Config {
        // Consumes self
        utils::bar();
        self.print_curr_state();
        println!("{}", config::EDIT_MESG);
        while let Some(a) = utils::get_item::<String>() {
            match a.as_str() {
                "k" => {
                    self.kill_time = interact::kill_time(Some(self.kill_time));
                }
                "w" => {
                    self.work_time = interact::work_time(Some(self.work_time));
                }
                "p" => {
                    self.password = interact::password(Some(&self.password));
                }
                "e" => {}
                "c" => {
                    self.print_curr_state();
                }
                _ => {}
            }
            println!("Select the next field to edit, please.");
        }
        self
    }
    pub fn remain(&self, beg: Instant) -> Config {
        Config::new_internal(
            self.kill_time,
            self.work_time - Instant::now().duration_since(beg).as_secs()/*60*/ as u16,
            self.password.clone(),
            self.processes.clone(),
        )
    }
    pub fn add_time(&self, time: u8) -> Config {
        Config::new_internal(
            self.kill_time,
            self.work_time + time as u16,
            self.password.clone(),
            self.processes.clone(),
        )
    }

    /// Helps the user create a config, providing sane defaults and a sense
    pub fn create() -> Config {
        println!("{}", config::CREATE_MESG);
        let kill_time = interact::kill_time(None);
        let work_time = interact::work_time(None);
        let passwd = interact::password(None);
        let processes = interact::processes();
        Config::new(kill_time, work_time, &passwd, processes)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
kill time: {} seconds
work time: {} minutes
password: {}
processes: {:?}
",
            self.kill_time, self.work_time, self.password, self.processes
        )
    }
}

impl GuessablePassword for Config {
    fn get_password(&self) -> String {
        self.password.clone()
    }
}
