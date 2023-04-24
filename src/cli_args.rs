use crate::config::Config;
use clap::Parser;
use directories::ProjectDirs;
use std::{fs, path::PathBuf, time::Duration};

pub type ArgInfo = (Config, PathBuf);
pub static FILENAME: &str = "focus-ami_0.2.1.yaml";

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(short, long, conflicts_with = "focus_period")]
    /// Edit the configuration
    pub config: bool,
    #[clap(conflicts_with = "config")]
    /// The time in minutes to focus for.
    pub focus_period: Option<u64>,
    /// Use the configuration at the path [CONFIG_PATH].
    pub config_path: Option<PathBuf>,
}

pub fn interpret_args(dirs: &ProjectDirs) -> Option<ArgInfo> {
    let cli = Cli::parse();
    let mut dirs = dirs.config_dir().to_path_buf();
    dirs.push(FILENAME);
    let path = cli.config_path.unwrap_or(dirs);
    let mut conf = if path.exists() {
        Config::get(&path).unwrap()
    } else {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        Config::create_at(&path)
    };
    if let Some(time) = cli
        .focus_period
        .and_then(|f_per| f_per.checked_mul(60))
        .map(Duration::from_secs)
    {
        conf.work_duration = time;
    }
    if cli.config {
        conf.usr_edit();
        conf.write(&path).unwrap();
        None
    } else {
        Some((conf, path))
    }
}
