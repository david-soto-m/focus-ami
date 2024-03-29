use std::collections::HashMap;
use std::process;
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt, Uid};

pub fn get_user(sys: &mut System) -> Option<&Uid> {
    sys.refresh_processes();
    sys.process(Pid::from(process::id() as usize))
        .unwrap()
        .user_id()
}

pub fn get_procs(system: &mut System) -> &HashMap<Pid, Process> {
    system.refresh_processes();
    system.processes()
}

pub fn diff_procs(
    proc_1: &HashMap<Pid, Process>,
    proc_2: &HashMap<Pid, Process>,
    user: Option<&Uid>,
) -> Vec<String> {
    println!("{user:?}");
    proc_2
        .iter()
        .filter_map(|(k, v)| {
            (!proc_1.contains_key(k) && (user.is_none() || v.user_id() == user))
                .then(|| v.name().to_owned())
        })
        .collect()
}
