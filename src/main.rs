use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx_to_killer, rx_to_killer) = mpsc::channel();
    let (tx_from_killer, rx_from_killer) = mpsc::channel();
    let join_vec = vec![
        thread::spawn(move || {
            focus_ami::killer(tx_from_killer, rx_to_killer);
        }),
        thread::spawn(move || {
            focus_ami::interact(tx_to_killer, rx_from_killer);
        }),
    ];
    join_vec.into_iter().for_each(|handle| {
        handle.join().unwrap();
    })
}
