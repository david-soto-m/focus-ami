use focus::config::Config;
use focus::killer;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let init_config = Config::get_init_config();

    let (tx_to_killer, rx_to_killer) = mpsc::channel();
    let (tx_from_killer, rx_from_killer) = mpsc::channel();
    let join = (
        thread::spawn(move || {
            killer::killer(tx_from_killer, rx_to_killer);
        }),
        "",
    );
    tx_to_killer.send(init_config.clone()).unwrap();
    thread::sleep(Duration::from_secs(1));
    tx_to_killer.send(Config::new(10, "", vec!["hi"])).unwrap();
    thread::sleep(Duration::from_secs(4));
    tx_to_killer.send(Config::new(5, "", vec!["hi"])).unwrap();
    rx_from_killer.recv().unwrap().unwrap();
    join.0.join().ok();
    // let s = System::new_all();
    // let proc_self = s.process(Pid::from(process::id() as i32)).unwrap();
}
