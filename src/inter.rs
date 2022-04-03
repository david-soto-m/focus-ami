use crate::config::Config;
use crate::utils::{self, errors, interact, Coms, GuessablePassword};
use std::collections::HashSet;
use std::sync::mpsc::Sender;
use std::time::Instant;

pub fn interact(
    string: String,
    tx: Sender<Coms>,
    mut config: Config,
    mut init_time: Instant,
) -> (Config, Instant) {
    match string.trim() {
        "e" => {
            if config.check_pass(None) {
                config = config.edit();
                tx.send(Coms::Message(config.clone(), None))
                    .expect(errors::COM);
            }
        }
        "p" => {
            if config.check_pass(None) {
                let config_rem = config.remain(init_time);
                tx.send(Coms::Message(
                    Config::new(1, u16::MAX, "".to_string(), HashSet::new()),
                    None,
                ))
                .expect(errors::COM);
                println!("{}", interact::PAUSE);
                utils::get_item::<String>();
                init_time = Instant::now();
                tx.send(Coms::Message(config_rem, Some(init_time)))
                    .expect(errors::COM);
                println!("{}", interact::CONT);
            }
        }
        "q" => {
            if config.check_pass(None) {
                println!(
                    "It will take up to {} seconds, but wont kill any processes you might have running",
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
            println!("{}", interact::ADD);
            if let Some(time) = utils::get_item() {
                tx.send(Coms::Message(config.add_time(time), None))
                    .expect(errors::COM);
            }
        }
        _ => {
            println!("{}", errors::INTER);
        }
    };
    (config, init_time)
}
