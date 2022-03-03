pub mod errors {
    pub static DIRECTORY: &str = "Couldn't create the config directory";
    pub static PROJECT: &str = "Couldn't infer the project directories";
    pub static R_FILE: &str = "Couldn't read from config file";
    pub static W_FILE: &str = "Couldn't write to config file";
    pub static ENCODING: &str = "Couldn't encode config to yaml";
    pub static DECODING: &str = "Couldn't decode config from yaml";
    pub static WRONG_ARG: &str = "is not a valid argument.\n Help:\t focus -h";
    pub static AQ: &str = "Unable to read or parse appropriately";
    pub static COM: &str = "Comunication error in channels, probably due to a thread failure";
    pub static PROC: &str = "Existential horror, couldn't find self!!!";
}

pub mod config {
    pub static FILENAME: &str = "config.yaml";
    pub static PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "focus");
}

pub mod help {
    pub static HELP: &str = "Usage: focus [OPTION]...
Regularly kill processes

  -a, a, --annotator        annotator mode, a guide to find processes names
  -A, A,                    annotator mode, a guide to find processes names,
                            without filtering for user,
  -f, f, --file [FILE]      use the config from FILE. FILE is a path starting
                            from your current working directory
  -s, s, --silent           no interactions allowed during the focus period
  -h, h, --help             show this help
";
}
