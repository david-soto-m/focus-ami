pub mod errors {
    pub static DIRECTORY: &str = "Couldn't create the config directory";
    pub static PROJECT: &str = "Couldn't infer the project directories";
}

pub mod config {
    pub static FILENAME: &str = "config.yaml";
    pub static PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "focus");
}
