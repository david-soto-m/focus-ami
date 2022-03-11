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
    pub static NEXT: &str = "Select the next field to edit, or quit (\\q)";
    pub static CREATE_MESG: &str = "We are going to create a config for you
Whenever you are happy with the default you can type \\q";
    pub static EDIT_MESG: &str = "Editing mode
You can edit parts or your config. Your available options are:
\t k: edit the time that passes between process killings
\t w: edit the work time
\t p: edit the password
\t e: edit the processes
\t c: check the config
\t\\q: stop editing";
}

pub mod interact {
    pub static INSTRUCTIONS: &str =
        "To start an interaction type the corresponding code, wait for up to one second
Possible interactions are:
\te: edit the config
\tp: pause
\tq: quit early
\tr: see remaining time
\ta: add some time to current run (but not to config)
";
    pub static ANNOTATOR: &str = "This can help you find the process name of your applications
This helper can fail in three ways
1. Your user is not the one responsible for the process created
2. The process was already started
3. The process is an instance of something else running it eg: python3
applications";
    pub static START: &str = "Please start the application you want to know the process name of.
Then press return.";
    pub static CAND: &str = "The possible candidates are: ";
    pub static PAUSE: &str = "Paused, pulse return to continue";
    pub static CONT: &str = "Finished pause";
    pub static ADD: &str = "How much time do you wish to add?
Max: 255";
    pub static FINNISH: &str = "\x07Finished";
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
