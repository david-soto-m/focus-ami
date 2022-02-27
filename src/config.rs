use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            stretch: 0,
            password: "".to_string(),
            processes: vec![],
        }
    }
}


//    println!("{}",serde_yaml::to_string(&Config::new(30, "Hola", vec!["hola"])).unwrap());
