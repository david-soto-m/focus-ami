use crate::consts::{config, errors};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    stretch: u64,
    password: String,
    pub processes: HashSet<String>,
}

impl Config {
    /// Creates a new config from the parameters
    pub fn new(stretch: u64, password: &str, processes: Vec<&str>) -> Self {
        Self {
            stretch,
            password: password.to_string(),
            processes: processes.iter().map(|slice| slice.to_string()).collect(),
        }
    }
    /// Get the config object from the config.yaml file.
    ///
    /// # Panics
    /// - if there is a bad yaml or if the default config results in a bad yaml
    /// - if access to the file is compromised anywhere in it's path.
    /// Lack of access to the config directory is distinguished from lack of access to the file
    pub fn get_init_config() -> Config {
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
        let proj_conf_file = proj_conf_dirs.join(config::FILENAME);
        if !proj_conf_file.exists() {
            fs::write(
                proj_conf_file.clone(),
                serde_yaml::to_string(&Config::new(5, "", vec!["vlc"])).expect(errors::ENCODING),
            )
            .expect(errors::W_FILE);
        }
        serde_yaml::from_str::<Config>(&fs::read_to_string(proj_conf_file).expect(errors::R_FILE))
            .expect(errors::DECODING)
    }

    /// returns a Duration from the stretch. It transforms from minutes to seconds
    pub fn get_stretch(&self) -> Duration {
        Duration::from_secs(self.stretch /* 60*/)
    }
}

//    println!("{}",serde_yaml::to_string(&Config::new(30, "Hola", vec!["hola"])).unwrap());
