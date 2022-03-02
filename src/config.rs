use crate::consts::{config, errors};
use crate::interact;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
            Config::write_config(&interact::create_config(), &filename);
            // is this idiomatic??
        }
        serde_yaml::from_str::<Config>(&fs::read_to_string(&filename).expect(errors::R_FILE))
            .unwrap_or_else(|err| {
                println!("{}", err);
                let config = interact::create_config();
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
}
