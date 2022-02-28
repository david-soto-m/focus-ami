pub mod errors {
    pub static DIRECTORY: &str = "Couldn't create the config directory";
    pub static PROJECT: &str = "Couldn't infer the project directories";
    pub static R_FILE: &str = "Couldn't read from config file";
    pub static W_FILE: &str = "Couldn't write to config file";
    pub static ENCODING: &str = "Couldn't encode config to yaml";
    pub static DECODING: &str = "Couldn't decode config from yaml";
}

pub mod config {
    pub static FILENAME: &str = "config.yaml";
    pub static PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "focus");
}
