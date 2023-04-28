use color_eyre::{
    eyre::{Context, ContextCompat, Result},
    Report,
};
use directories::ProjectDirs;
use std::{sync::mpsc, thread, time::Duration};
mod annotator;
mod cli_args;
mod comms;
mod config;
mod killer;
mod user;
use comms::MainToFromThreads;

fn main() -> Result<()> {
    color_eyre::install()?;
    let proj_dirs = ProjectDirs::from("org", "amisoft", "focus-ami")
        .context("failed to get the configuration directories for your platform")?;
    // Get the arguments and maybe check out if in annotation mode
    let Some(arg_info) = cli_args::interpret_args(&proj_dirs).context("failed to properly parse arguments")? else{ return Ok(()) };
    // main <-> killer
    let (tx_m2k, rx_m2k) = mpsc::channel();
    let (tx_k2m, rx_k2m) = mpsc::channel();
    // main            <->interactor
    let (tx_m2i, rx_m2i) = mpsc::channel();
    let (tx_i2m, rx_i2m) = mpsc::channel::<MainToFromThreads>();
    //          killer -> interactor
    let (tx_i2k, rx_i2k) = mpsc::channel();
    // Set up the killing task
    let killer_handle = thread::spawn(move || {
        killer::killer(&tx_k2m, &rx_m2k, &rx_i2k)?;
        Ok::<(), Report>(())
    });
    // Set up the interactive task
    let interactor_handle = thread::spawn(move || {
        user::thread(&tx_i2m, &tx_i2k, &rx_m2i, arg_info)?;
        Ok::<(), Report>(())
    });
    // see if somebody says to die, and then kill the other
    loop {
        if rx_k2m.try_recv().is_ok() {
            tx_m2i
                .send(MainToFromThreads)
                .context("failed to send kill message to user thread from main")?;
            break;
        }
        if rx_i2m.try_recv().is_ok() {
            tx_m2k
                .send(MainToFromThreads)
                .context("failed to send kill message to killer thread from main")?;
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }
    killer_handle.join().unwrap()?;
    interactor_handle.join().unwrap()?;
    Ok(())
}
