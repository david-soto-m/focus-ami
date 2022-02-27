use sysinfo::{ProcessExt, System, SystemExt};
pub mod config;

fn kill() {
    let s = System::new_all();
    s.processes()
        .iter()
        .filter(|(_, proc)| proc.name() == "firefox-bin")
        .for_each(|(_, proc)| {
            proc.kill();
        });
}
