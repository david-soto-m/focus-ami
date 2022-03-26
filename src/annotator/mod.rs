use crate::utils;
use crate::utils::{errors, interact};
use std::collections::HashSet;
use std::process;
use sysinfo::{Pid, ProcessExt, System, SystemExt, Uid, UserExt};

pub fn annotator(filter_users: bool) {
    let s = System::new_all();
    println!("{}", interact::ANNOTATOR);
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
    println!("{}", interact::START);
    utils::get_item::<String>();
    println!("{}", interact::CAND);
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
