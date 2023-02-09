use focus_ami::cli::{self, InteractType};
use focus_ami::config::Config;
use focus_ami::inter;
use focus_ami::utils::{self, errors, interact, Coms};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (mut config, interactive, path) = cli::interpret_args();
    // if you are in annotator mode, do nothing
    if config == Config::default() {
        return;
    }
    let (tx, rx_to_killer) = mpsc::channel();
    let (tx_from_killer, rx) = mpsc::channel();
    let killer_handle = thread::spawn(move || {
        focus_ami::killer(&tx_from_killer, &rx_to_killer);
    });
    let mut init_time = Instant::now();
    tx.send(Coms::Message(config.clone(), Some(init_time)))
        .expect(errors::COM);
    match interactive {
        InteractType::NormalRun => {
            config.print_curr_state();
            println!("{}", interact::INSTRUCTIONS);
            utils::bar();
            let (interuptor, interruptee) = mpsc::channel();
            let int = interuptor.clone();
            thread::spawn(move || {
                utils::async_string(&int);
            });
            //I don't really care what happens to it, only if it produces a value
            while rx.try_recv().is_err() {
                thread::sleep(Duration::from_millis(500));
                if let Ok(string) = interruptee.try_recv() {
                    (config, init_time) = inter::interact(&string, &tx.clone(), config, init_time);
                    // handle the petition, then spawn another
                    let int = interuptor.clone();
                    thread::spawn(move || {
                        utils::async_string(&int);
                    });
                    utils::bar();
                };
            }
            println!("{}", interact::FINNISH);
        }
        InteractType::SilentRun => {
            rx.recv().expect(errors::COM);
        }
    };
    tx.send(Coms::End).expect(errors::COM);
    killer_handle.join().unwrap();
    config.write_config(&path);
}
