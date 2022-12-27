use crate::annotator;
use crate::Config;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(long, short)]
    /// Use the configuration from <CONFIG>. <CONFIG> is a path starting from
    /// your current working directory
    pub config: Option<PathBuf>,
    /// Disallow interactions during the focus period
    #[clap(short, long)]
    pub silent: bool,
    /// Annotator mode, a guide to find processes names
    #[clap(short, long)]
    pub annotate: bool,
    /// Annotator mode, a guide to find processes names, without filtering for user
    #[clap(long)]
    pub annotate_no_user: bool,
}

pub enum InteractType {
    NormalRun,
    SilentRun,
}

pub fn interpret_args() -> (Config, InteractType, PathBuf) {
    let cli = Cli::parse();
    if cli.annotate | cli.annotate_no_user {
        annotator::annotator(!cli.annotate_no_user);
        (Config::default(), InteractType::NormalRun, PathBuf::new())
    } else {
        let (config, path) = Config::get_or_create(cli.config);
        let interactive = match cli.silent {
            true => InteractType::SilentRun,
            false => InteractType::NormalRun,
        };
        (config, interactive, path)
    }
}
