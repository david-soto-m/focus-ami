use crate::config::Config;
use crate::utils::{errors, Coms};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};
mod annotator;
pub mod cli;
pub mod config;
pub mod inter;
pub mod utils;

/// This function is the one that sends the kill signals to the processes
/// It waits for a Config to be sent to it to start blocking.
///
/// It runs on a while that
/// 1. checks if you have run out of time
/// 1. Sees if config has changed
/// 1. gets all running processes
/// 1. kills those in the processes list of the running configuration
/// 1. sleeps
pub fn killer(tx: Sender<()>, rx: Receiver<Coms>) {
    let (mut config, mut init_time);
    if let Coms::Message(conf, Some(time)) = rx.recv().expect(errors::COM) {
        (config, init_time) = (conf, time);
    } else {
        panic!("{}", errors::COM)
    }
    while Instant::now().duration_since(init_time) < config.get_work_time() {
        if let Ok(Coms::Message(conf, time)) = rx.try_recv() {
            config = conf;
            if let Some(now) = time {
                init_time = now;
            }
        }
        let s = System::new_all();
        s.processes()
            .iter()
            .filter(|(_, process)| config.contains(process.name()))
            .for_each(|(_, process)| {
                process.kill();
            });
        thread::sleep(config.get_kill_time());
    }
    tx.send(()).expect(errors::COM);

    loop {
        if let Coms::End = rx.recv().expect(errors::COM) {
            break;
        }
    }
}
