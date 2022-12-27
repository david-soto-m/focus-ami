use crate::utils::{self, errors, interact};
use std::collections::HashSet;
use std::process;
use sysinfo::{Pid, ProcessExt, System, SystemExt, Uid, UserExt};

pub fn annotator(mut filter_users: bool) {
    let s = System::new_all();
    println!("{}", interact::ANNOTATOR);
    let proc_self = s
        .process(Pid::from(process::id() as usize))
        .expect(errors::PROC);
    let user_id = proc_self.user_id();
    if filter_users && user_id.is_none() {
        println!("Disabling filter because no user id could be established");
        filter_users = false;
    };
    if filter_users {
        println!(
            "We are selecting the user {} with {:?} form these\n{:?}",
            match s.get_user_by_id(user_id.unwrap()) {
                Some(user) => user.name(),
                None => "with unknown name and",
            },
            user_id.unwrap(), //is checked to be some beforehand
            s.users()
                .iter()
                .map(|user| (user.name(), user.id()))
                .collect::<Vec<(&str, &Uid)>>()
        );
    }
    let mut a = HashSet::new();
    s.processes()
        .iter()
        .filter(|(_, proc)| {
            if filter_users {
                user_id == proc.user_id()
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
                user_id == proc.user_id()
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
