use crate::comms::{InteractToKiller, MainToFromThreads};
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Instant,
};
use sysinfo::{ProcessExt, System, SystemExt};

/// This function is the one that sends the kill signals to the processes
/// It waits for a Config to be sent to it to start blocking.
///
/// It runs on a while that
/// 1. checks if you have run out of time
/// 2. Sees there is a message from the user
/// 1. gets all running processes
/// 1. kills those in the processes list of the running configuration
/// 1. sleeps
/// # Panics
/// When there are communication errors between threads
pub fn killer(
    tx_to_main: &Sender<MainToFromThreads>,
    rx_from_main: &Receiver<MainToFromThreads>,
    rx_from_user: &Receiver<InteractToKiller>,
) {
    let init_time = Instant::now();
    let InteractToKiller::Config(mut config) = rx_from_user.recv().unwrap() else {
        panic!("") //TODO Not correct error!
    };
    let mut dur = config.work_duration;
    //while no new main msgs ↓ and not enough time has passed since init_time ↓  run
    while rx_from_main.try_recv().is_err() && init_time.elapsed() < dur {
        check_user_messages(rx_from_user, &mut dur, &mut config);
        let s = System::new_all();
        s.processes()
            .iter()
            .filter(|(_, process)| config.contains(process.name()))
            .for_each(|(_, process)| {
                process.kill();
            });
        thread::sleep(config.kill_period);
    }
    tx_to_main.send(MainToFromThreads).unwrap();
}

/// Handles user messages
///
/// When a `InteractToKiller::Pause` message arrives it locks the thread until a
/// `InteractToKiller::Time` message is received, updates the local duration and
/// then releases the thread.
///
/// When a `InteractToKiller::Time` message is received the local duration is
/// updated
///
/// When a `InteractToKiller::Config` message is recieved, the local copy of the
/// `config` is updated.
fn check_user_messages(
    rx_from_user: &Receiver<InteractToKiller>,
    dur: &mut std::time::Duration,
    config: &mut crate::config::Config,
) {
    if let Ok(message) = rx_from_user.try_recv() {
        match message {
            InteractToKiller::Pause => {
                if let InteractToKiller::Time(d) = rx_from_user.recv().unwrap() {
                    *dur = d;
                }
            }
            InteractToKiller::Time(d) => {
                *dur = d;
            }
            InteractToKiller::Config(conf) => {
                *config = conf;
            }
        }
    }
}
