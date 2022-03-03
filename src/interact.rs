use crate::config::Config;
use crate::consts::{errors, help};
use std::collections::HashSet;
use std::env;
use std::process;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Instant;
use sysinfo::{Pid, ProcessExt, System, SystemExt, Uid, UserExt};
pub mod utils;

fn help() {
    println!("{}", help::HELP);
}

//TODO
fn anotator(filter_users: bool) {
    let s = System::new_all();
    println!(
        "This can help you find the process name of your applications
This helper can fail in three ways
1. Your user is not the one responsible for the process created
2. The process was already started
3, The process is an instance of something else running it eg: python3 applications"
    );
    let proc_self = s.process(Pid::from(process::id() as i32)).expect(errors::PROC);
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
        "Please start the application you want to know the process name of and then press return"
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
                println!("{}, {}", x, errors::WRONG_ARG);
                interactive = false;
                config = Config::default();
            }
        }
    }
    (config, interactive)
}

/// the interaction loop
pub fn interact(tx: Sender<(Config, Option<Instant>)>, rx: Receiver<()>) {
    let (init_config, interactive) = interpret_args();

    if interactive {
        println!("Starting with config: {}", init_config);
    }
    tx.send((init_config.clone(), Some(Instant::now())))
        .expect(errors::COM);
    // TODO interaction loop -> edit config, quit early, pause or snooze
    rx.recv().expect(errors::COM);
    if interactive {
        println!("\x07Finished");
    }
}
