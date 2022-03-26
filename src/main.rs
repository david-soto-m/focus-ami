use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        focus_ami::interact();
    });
    handle.join().unwrap();
}
