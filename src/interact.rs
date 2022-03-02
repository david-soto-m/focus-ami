use crate::config::Config;
use crate::consts::errors;
use std::env;
use std::io;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};

// TODO
fn help() {
    println!("Not done yet");
}

//TODO
fn anotator() {
    println!("Not done yet");
    /*
    use std::process;
    // let s = System::new_all();
    // let proc_self = s.process(Pid::from(process::id() as i32)).unwrap();

      s.processes()
            .iter()
            .filter(|(_, proc)| proc.uid == proc_self.uid)
            .for_each(|(_, proc)| {
                // proc.kill();
                // println!("{:?}", proc.name());
            });
        println!("{:?}" ,s.users());

    */
}

pub fn create_config() -> Config {
    println!("We are going to create a config for you");
    println!("Whenever you are happy with the default type \\q");
    println!("The first item is the period after which to kill processes.");
    println!("number is interpreted as seconds");
    println!("By default: 30 seconds || Max: 255 s || Min 1");
    let interval = match get_item() {
        Some(num) => {
            if num > 0 {
                num
            } else {
                1
            }
        }
        None => 30,
    };
    println!("Now we are going to determine the minutes you want to focus");
    println!("Keep in mind that you will have to wait for the whole period.");
    println!("By default: 30 minutes || Max: 65535");
    let stretch = get_item().unwrap_or(30);
    println!("The next step is setting up a password.");
    println!("This is not for security, but in order to be a nuisance");
    println!("That way you may be discouraged to quit the second you get distracted");
    println!("By default: a || Recommended : aslkdhgjhkadbchjqwmepam ionk");
    let passwd = match get_item() {
        Some(stri) => stri,
        None => "a".to_string(),
    };
    println!("Finally we are going to set up the processes to be blocked");
    println!("If you need help to get the name of a process please");
    println!("1. When you only have unknown processes type \\q");
    println!("2. Run \"focus -a\". Alternatively use ps -e (and diff?) to figure out the name of your process");
    println!("3. Add it to your config during a normal run");
    println!("To add a process type it and press enter");
    println!("To stop adding processes just type \\q");
    let processes = get_vec(get_item);
    Config::new(interval, stretch, &passwd, processes)
}

fn interpret_args() -> (Config, bool) {
    let mut args = env::args();
    args.next(); // ignore
    let mut interactive = true;
    let mut config = Config::get_or_create(None);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "F" | "f" | "-c" | "-C" | "c" => {
                config = Config::get_or_create(args.next());
            }
            "-s" | "s" | "S" => {
                interactive = false;
            }
            "--help" | "help" | "-h" | "h" => {
                help();
                interactive = false;
                config = Config::default();
            }
            "-a" | "A" | "a" => {
                anotator();
                interactive = false;
                config = Config::default();
            }
            x => {
                println!("{}, {}", x, errors::WRONG_ARG);
                interactive = false;
                config = Config::default();
            }
        }
    }
    (config, interactive)
}

pub fn interact(tx: Sender<Config>, rx: Receiver<()>) {
    let (init_config, _interactive) = interpret_args();

    tx.send(init_config.clone()).unwrap();
    println!("{:?}", init_config);

    rx.recv().unwrap();
    println!("\x07Finished");
}

/// Gets [anything that implements FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)
///
///
fn get_item<T>() -> Option<T>
where
    T: FromStr,
{
    loop {
        let mut attempt = String::new();
        io::stdin().read_line(&mut attempt).expect(errors::AQ);
        match attempt.trim() {
            "" | "\\q" => break None,
            item => match item.parse::<T>() {
                Ok(item) => break Some(item),
                Err(_) => {
                    println!("{},", errors::AQ);
                    continue;
                }
            },
        }
    }
}

/// Runs the selected
fn get_vec<T, U>(function: T) -> Vec<U>
where
    T: Fn() -> Option<U>,
{
    let mut vec: Vec<U> = Vec::new();
    loop {
        match function() {
            Some(elem) => vec.push(elem),
            None => break vec,
        }
    }
}
