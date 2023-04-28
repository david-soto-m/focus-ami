use crate::config::Config;
use clap::Parser;
use color_eyre::{
    eyre::{Context, ContextCompat, Result},
    Section,
};
use directories::ProjectDirs;
use std::{fs, path::PathBuf, time::Duration};
pub type ArgInfo = (Config, PathBuf);
pub static FILENAME: &str = "focus-ami_1.yaml";

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
    #[clap(short, long)]
    pub path: Option<PathBuf>,
}

pub fn interpret_args(dirs: &ProjectDirs) -> Result<Option<ArgInfo>> {
    let cli = Cli::parse();
    let mut dirs = dirs.config_dir().to_path_buf();
    dirs.push(FILENAME);
    let path = cli.path.unwrap_or(dirs);
    let mut conf = if path.exists() {
        Config::get(&path)?
    } else {
        fs::create_dir_all(
            path.parent()
                .context(format!("{path:?} parent couldnt be constructed"))?,
        )
        .context(format!("{:?} couldn't be created", path.parent().unwrap()))
        .suggestion(format!(
            "Check the write permissions of all the parents of {path:?} starting from the bottom"
        ))?;
        Config::create_at(&path)?
    };
    if let Some(time) = cli
        .focus_period
        .and_then(|f_per| f_per.checked_mul(60))
        .map(Duration::from_secs)
    {
        conf.work_duration = time;
    }
    Ok(if cli.config {
        conf.usr_edit()
            .context("failed to edit the configuration")?;
        conf.write(&path)?;
        None
    } else {
        Some((conf, path))
    })
}
