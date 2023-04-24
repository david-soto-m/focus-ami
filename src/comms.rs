use crate::config::Config;
use std::time::Duration;

/// Any communications to and from main is a kys suggestion.
pub struct MainToFromThreads;

/// Possible things that the user might want to inform the killing part of the
/// program
pub enum InteractToKiller {
    Config(Config),
    Time(Duration),
    Pause,
}
