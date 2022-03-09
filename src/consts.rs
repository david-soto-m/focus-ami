pub mod errors {
    pub static DIRECTORY: &str = "Couldn't create the config directory";
    pub static PROJECT: &str = "Couldn't infer the project directories";
    pub static R_FILE: &str = "Couldn't read from config file";
    pub static W_FILE: &str = "Couldn't write to config file";
    pub static ENCODING: &str = "Couldn't encode config to yaml";
    pub static DECODING: &str = "Couldn't decode config from yaml";
    pub static ARG: &str = "is not a valid argument.\nHelp:\t focus -h";
    pub static AQ: &str = "Unable to read or parse appropriately";
    pub static COM: &str = "Comunication error in channels, probably due to a thread failure";
    pub static PROC: &str = "Existential horror, couldn't find self!!!";
    pub static INTER: &str = "is not a valid interaction";
}

pub mod config {
    pub static FILENAME: &str = "config.yaml";
    pub static PROJECT_INFO: (&str, &str, &str) = ("org", "amisoft", "focus");
    pub static CREATE_MESG: &str = "We are going to create a config for you
Whenever you are happy with the default type \\q";
    pub static EDIT_MESG: &str = "\t k: edit the time that passes between process killings
\t w: edit the work time
\t p: edit the password
\t e: edit the processes
\t c: check the config
\t\\q: stop editing";
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
