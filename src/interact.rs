use crate::config::Config;
use crate::consts::errors;
use std::sync::mpsc::{Receiver, Sender};
// use std::thread;
// use std::time::Duration;

use std::env;
// TODO
pub fn help() {}

//TODO
fn anotator() {
/*
use std::process;
// let s = System::new_all();
// let proc_self = s.process(Pid::from(process::id() as i32)).unwrap();

  s.processes()
        .iter()
        .filter(|(_, proc)| proc.uid == proc_self.uid)
        .for_each(|(_, proc)| {
            // proc.kill();
            // println!("{:?}", proc.name());
        });
    println!("{:?}" ,s.users());

*/
}

fn interpret_args() -> (Config, bool) {
    let mut args = env::args();
    args.next(); // ignore
    let mut interactive = true;
    let mut config = Config::get_or_create(None);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "F" | "f" | "-c" | "-C" | "c" => {
                config = Config::get_or_create(args.next());
            }
            "-s" | "s" | "S" => {
                interactive = false;
            }
            "--help" | "help" | "-h" | "h" => {
                help();
                interactive = false;
                config = Config::new(0,"",vec![]);
            }
            "-a" | "A" | "a" => {
                anotator();
                interactive = false;
                config = Config::new(0,"",vec![]);
            }
            x => {
                println!("{}, {}", x, errors::WRONG_ARG);
                interactive = false;
                config = Config::new(0,"",vec![]);
            }
        }
    }
    (config, interactive)
}

pub fn interact(tx: Sender<Config>, rx: Receiver<()>) {
    let (init_config, _interactive) = interpret_args();

    tx.send(init_config.clone()).unwrap();

    rx.recv().unwrap();
}
