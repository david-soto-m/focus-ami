use crate::config::Config;
use crate::utils::{errors, interact, GuessablePassword};
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{ProcessExt, System, SystemExt};
mod cli;
mod config;
mod utils;
mod anotator;

/// The two types of communications are end or a config, This is an enum that
/// reflects that reality
pub enum Coms {
    Message(crate::config::Config, Option<Instant>),
    End,
}

/// # The interaction loop
///
/// It spawns another thread that is on the lookout for user input synchronously.
/// Meanwhile this loop is checking if any of the threads (killer and
/// synchronous input) have sent anything back.
///
/// When the killing loop send a unit type it means it has finished, and
/// whenever this thread is free it should quit.
///
/// When the synchronous interaction returns a letter, it is processed and this
/// thread asumes control over the interactions synchronously until it is
/// finished. When it is finished, this thread spawns the next synchronous one
///
/// When in silent mode, it doesn't spawn the thread and it just waits for the
/// killing thread to be done, synchronously,
pub fn interact(tx: Sender<Coms>, rx: Receiver<()>) {
    let (mut config, interactive, path) = cli::interpret_args();
    let mut init_time = Instant::now();
    tx.send(Coms::Message(config.clone(), Some(init_time)))
        .expect(errors::COM);
    if interactive {
        config.print_curr_state();
        println!("{}", interact::INSTRUCTIONS);
        utils::bar();
        let (interuptor, interruptee) = mpsc::channel();
        let int = interuptor.clone();
        thread::spawn(move || {
            utils::async_string(int);
        });
        //I don't really care what happens to it, only if it produces a value
        while rx.try_recv().is_err() {
            thread::sleep(Duration::from_millis(500));
            if let Ok(string) = interruptee.try_recv() {
                match string.trim() {
                    "e" => {
                        if config.check_pass(None) {
                            config = config.edit();
                            tx.send(Coms::Message(config.clone(), None))
                                .expect(errors::COM);
                        }
                    }
                    "p" => {
                        if config.check_pass(None) {
                            let config_rem = config.remain(init_time);
                            tx.send(Coms::Message(
                                Config::new(1, u16::MAX, "".to_string(), HashSet::new()),
                                None,
                            ))
                            .expect(errors::COM);
                            println!("{}", interact::PAUSE);
                            utils::get_item::<String>();
                            init_time = Instant::now();
                            tx.send(Coms::Message(config_rem, Some(init_time)))
                                .expect(errors::COM);
                            println!("{}", interact::CONT);
                        }
                    }
                    "q" => {
                        if config.check_pass(None) {
                            println!(
                                "It will take up to {} seconds, but wont kill any processes you might have running",
                                config.get_kill_time_as_seconfs()
                            );
                            tx.send(Coms::Message(Config::default(), None))
                                .expect(errors::COM);
                        }
                    }
                    "r" => {
                        println!(
                            "{} minutes remaining",
                            config.remain(init_time).get_work_time_as_min()
                        );
                    }
                    "a" => {
                        println!("{}", interact::ADD);
                        if let Some(time) = utils::get_item() {
                            tx.send(Coms::Message(config.add_time(time), None))
                                .expect(errors::COM);
                        }
                    }
                    _ => {
                        println!("{}", errors::INTER);
                    }
                };
                // handle the petition, then spawn another
                let int = interuptor.clone();
                thread::spawn(move || {
                    utils::async_string(int);
                });
                utils::bar();
            };
        }
        println!("{}", interact::FINNISH);
    } else {
        rx.recv().expect(errors::COM);
    }
    tx.send(Coms::End).expect(errors::COM);
    config.write_config(&path);
}

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
