use sysinfo::{ProcessExt, System, SystemExt};
pub mod config;
pub mod consts;

pub fn kill() {
    let s = System::new_all();
    s.processes()
        .iter()
        .filter(|(_, proc)| proc.name() == "firefox-bin")
        .for_each(|(_, proc)| {
            proc.kill();
        });
}

/*
  s.processes()
        .iter()
        .filter(|(_, proc)| proc.uid == proc_self.uid)
        .for_each(|(_, proc)| {
            // proc.kill();
            // println!("{:?}", proc.name());
        });
    println!("{:?}" ,s.users());

*/
