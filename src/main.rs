use directories::ProjectDirs;
use std::{sync::mpsc, thread, time::Duration};

mod annotator;
mod cli_args;
mod comms;
mod config;
mod killer;
mod user;
use comms::MainToFromThreads;

fn main() {
    let proj_dirs = ProjectDirs::from("org", "amisoft", "focus-ami").unwrap();
    // Get the arguments and maybe check out if in annotation mode
    let Some(arg_info) = cli_args::interpret_args(&proj_dirs) else{ return };
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
        killer::killer(&tx_k2m, &rx_m2k, &rx_i2k);
    });
    // Set up the interactive task
    let interactor_handle = thread::spawn(move || {
        user::thread(&tx_i2m, &tx_i2k, &rx_m2i, arg_info);
    });
    // see if somebody says to die, and then kill the other
    loop {
        if rx_k2m.try_recv().is_ok() {
            tx_m2i.send(MainToFromThreads).unwrap();
            break;
        }
        if rx_i2m.try_recv().is_ok() {
            tx_m2k.send(MainToFromThreads).unwrap();
            break;
        }
        thread::sleep(Duration::from_millis(500));
    }
    killer_handle.join().unwrap();
    interactor_handle.join().unwrap();
    println!("\x07Finished");
}
