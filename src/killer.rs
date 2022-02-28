// use std::thread;
use crate::config::Config;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

/// This function is the one that sends the kill signals to the processes
/// It waits for a Config to be sent to it to start blocking,
/// the first thing it does is to check if the config has changed
pub fn killer(tx: Sender<Result<(), ()>>, rx: Receiver<Config>) {
    let mut config = rx.recv().unwrap();
    let init_time = Instant::now();
    while Instant::now().duration_since(init_time) < config.get_stretch() {
        match rx.try_recv() {
            Ok(conf) => {
                config = conf;
            }
            Err(_) => (),
        };
        let s = System::new_all();
        s.processes()
            .iter()
            .filter(|(_, process)| config.processes.contains(process.name()))
            .for_each(|(_, process)| {process.kill();});
        thread::sleep(Duration::from_secs(1));
    }
    tx.send(Ok(())).unwrap();
}

/*

use std::process;

pub fn kill() {
    let s = System::new_all();
    s.processes()
        .iter()
        .filter(|(_, proc)| proc.name() == "firefox-bin")
        .for_each(|(_, proc)| {
            proc.kill();
        });
}
  s.processes()
        .iter()
        .filter(|(_, proc)| proc.uid == proc_self.uid)
        .for_each(|(_, proc)| {
            // proc.kill();
            // println!("{:?}", proc.name());
        });
    println!("{:?}" ,s.users());

*/
