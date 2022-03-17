use clap::Parser;
use std::path::PathBuf;
use crate::Config;
use crate::anotator;

#[derive(Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(long, short, parse(from_os_str))]
    /// Use the configuration from <CONFIG>. <CONFIG> is a path starting from
    /// your current working directory
    pub config: Option<PathBuf>,
    /// Disallow interactions during the focus period
    #[clap(short, long)]
    pub silent: bool,
    /// Annotator mode, a guide to find processes names
    #[clap(short, long)]
    pub anotate: bool,
    /// Annotator mode, a guide to find processes names, without filtering for user
    #[clap(long)]
    pub anotate_no_user: bool,
}

pub fn interpret_args() -> (Config, bool, PathBuf) {
    let cli = Cli::parse();
    let interactive = !cli.silent;
    let (mut config, path);
    (config, path) = Config::get_or_create(cli.config);
    if cli.anotate | cli.anotate_no_user {
        config = Config::default();
        anotator::anotator(!cli.anotate_no_user);
    }
    (config, interactive, path)
}
