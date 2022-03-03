// use std::thread;
use crate::config::Config;
use crate::consts::errors;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

/// This function is the one that sends the kill signals to the processes
/// It waits for a Config to be sent to it to start blocking.
///
/// It runs on a while that
/// * checks if you have run out of time **then**
/// 1. Sees if config has changed
/// 1. gets all running processes
/// 1. kills those in the processes list of the running configuration
/// 1. sleeps
pub fn killer(tx: Sender<()>, rx: Receiver<(Config, Option<Instant>)>) {
    let (mut config, opts) = rx.recv().unwrap();
    let mut init_time = match opts {
        Some(now) => now,
        None => panic!("{}", errors::COM),
    };
    while Instant::now().duration_since(init_time) < config.get_stretch() {
        if let Ok((conf, time)) = rx.try_recv() {
            config = conf;
            if let Some(now) = time {
                init_time = now;
            }
        }
        let s = System::new_all();
        s.processes()
            .iter()
            .filter(|(_, process)| config.processes.contains(process.name()))
            .for_each(|(_, process)| {
                process.kill();
            });
        thread::sleep(config.get_interval());
    }
    tx.send(()).expect(errors::COM);
}
