use crate::consts::{config, errors};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    stretch: u8,
    password: String,
    processes: Vec<String>,
}

impl Config {
    pub fn new(stretch: u8, password: &str, processes: Vec<&str>) -> Self {
        Self {
            stretch,
            password: password.to_string(),
            processes: processes.iter().map(|slice| slice.to_string()).collect(),
        }
    }
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
                serde_yaml::to_string(&Config::default()).unwrap(),
            )
            .unwrap();
        }
        serde_yaml::from_str::<Config>(&fs::read_to_string(proj_conf_file).unwrap()).unwrap()
    }
}

//    println!("{}",serde_yaml::to_string(&Config::new(30, "Hola", vec!["hola"])).unwrap());
