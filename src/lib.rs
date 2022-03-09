use crate::config::Config;
use crate::consts::{errors, help};
use crate::utils::GuessablePassword;
use std::collections::HashSet;
use std::env;
use std::process;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use sysinfo::{Pid, ProcessExt, System, SystemExt, Uid, UserExt};
pub mod config;
pub mod consts;
pub mod utils;

pub enum Coms {
    Message(crate::config::Config, Option<Instant>),
    End,
}

fn help() {
    println!("{}", help::HELP);
}

fn anotator(filter_users: bool) {
    let s = System::new_all();
    println!(
        "This can help you find the process name of your applications
This helper can fail in three ways
1. Your user is not the one responsible for the process created
2. The process was already started
3, The process is an instance of something else running it eg: python3
applications"
    );
    let proc_self = s
        .process(Pid::from(process::id() as i32))
        .expect(errors::PROC);
    if filter_users {
        println!(
            "We are selecting the user with uid {} form these\n{:?}",
            proc_self.uid,
            s.users()
                .iter()
                .map(|user| (user.name(), user.uid()))
                .collect::<Vec<(&str, Uid)>>()
        );
    }
    let mut a = HashSet::new();
    s.processes()
        .iter()
        .filter(|(_, proc)| {
            if filter_users {
                proc.uid == proc_self.uid
            } else {
                true
            }
        })
        .for_each(|(_, proc)| {
            a.insert(proc.name());
        });
    println!(
        "Please start the application you want to know the process name of and then
press return"
    );
    utils::get_item::<String>();
    println!("The possible candidates are: ");
    System::new_all()
        .processes()
        .iter()
        .filter(|(_, proc)| {
            if filter_users {
                proc.uid == proc_self.uid
            } else {
                true
            }
        })
        .for_each(|(_, proc)| {
            if !a.contains(proc.name()) {
                println!("\t- {}", proc.name());
            }
        });
}

fn interpret_args() -> (Config, bool) {
    let mut args = env::args();
    args.next(); // ignore
    let mut interactive = true;
    let mut config = Config::get_or_create(None);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "f" | "--file" => {
                config = Config::get_or_create(args.next());
            }
            "-s" | "s" | "--silent" => {
                interactive = false;
            }
            "--help" | "-h" | "h" => {
                help();
                interactive = false;
                config = Config::default();
            }
            "-a" | "a" | "--annotator" => {
                anotator(true);
                interactive = false;
                config = Config::default();
            }
            "-A" | "A" => {
                anotator(false);
                interactive = false;
                config = Config::default();
            }
            x => {
                println!("{} {}", x, errors::ARG);
                interactive = false;
                config = Config::default();
            }
        }
    }
    (config, interactive)
}

/// the interaction loop
pub fn interact(tx: Sender<Coms>, rx: Receiver<()>) {
    let (mut config, interactive) = interpret_args();
    let mut init_time = Instant::now();
    tx.send(Coms::Message(config.clone(), Some(init_time)))
        .expect(errors::COM);
    if interactive {
        config.print_curr_state();
        println!(
            "To start an interaction type the corresponding code, wait for up to one second
Possible interactions are:
\te: edit the config
\tp: pause
\tq: quit early
\tr: see remaining time
\ta: add some time to current run (but not to config)
"
        );
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
                        if config.check_pass() {
                            config = config.edit();
                            tx.send(Coms::Message(config.clone(), None))
                                .expect(errors::COM);
                        }
                    }
                    "p" => {
                        if config.check_pass() {
                            config = config.remain(init_time);
                            tx.send(Coms::Message(Config::new(1, u16::MAX, "", vec![]), None))
                                .expect(errors::COM);
                            println!("Paused, pulse return to continue");
                            utils::get_item::<String>();
                            init_time = Instant::now();
                            tx.send(Coms::Message(config.clone(), Some(init_time)))
                                .expect(errors::COM);
                            println!("Finished pause");
                        }
                    }
                    "q" => {
                        if config.check_pass() {
                            println!(
                                "It will take up to {} seconds",
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
                        println!("How much time do you wish to add?");
                        println!("Max: 255");
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
        println!("\x07Finished");
    } else {
        rx.recv().expect(errors::COM);
    }
    tx.send(Coms::End).expect(errors::COM); // this has to be unique
}

/// This function is the one that sends the kill signals to the processes
/// It waits for a Config to be sent to it to start blocking.
///
/// It runs on a while that
/// * checks if you have run out of time **then**
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
            .filter(|(_, process)| config.processes.contains(process.name()))
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
