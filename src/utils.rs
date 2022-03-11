use crate::consts::errors;
use std::collections::HashSet;
use std::io;
use std::str::FromStr;
use std::sync::mpsc::Sender;

/// Gets [anything that implements FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)
/// or q
pub fn get_item<T>() -> Option<T>
where
    T: FromStr,
{
    loop {
        let mut attempt = String::new();
        io::stdin().read_line(&mut attempt).expect(errors::AQ);
        match attempt.trim() {
            "\\q" => break None,
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

/// Gets a vector that stops at a none parameter.
/// It is append only.
/// It must be provided with a closure (or function) that returns an option
/// You can get such a thing from a function that returns error using
/// ```rust
/// # use focus::interact::get_vec;
/// # fn returns_error()->Result< (), () >{
/// #     Err(())
/// # }
/// # let a =
/// get_vec(||{returns_error().ok()})
/// # ;
/// # assert_eq!(a.len(), 0);
/// ```
pub fn get_vec<T, U>(function: T) -> Vec<U>
where
    T: Fn(&mut Vec<U>) -> Option<U>,
{
    let mut vec: Vec<U> = Vec::new();
    loop {
        match function(&mut vec) {
            Some(item) => vec.push(item),
            None => break vec,
        }
    }
}

/// Adds or removes processes from the current processes HashSet
/// It admits four types of interactions
/// 1. `\q`: Stop adding processes
/// 1. `\w`: See the processes list
/// 1. `rm process name`: remove a process from the list
/// 1. `process name`: add a process to the list
pub fn get_proc(mut set: HashSet<String>) -> HashSet<String> {
    loop {
        let mut attempt = String::new();
        io::stdin().read_line(&mut attempt).expect(errors::AQ);
        match attempt.trim() {
            "\\q" => break set,
            "\\w" => println!("{:?}", set),
            item => match item.parse::<String>() {
                Ok(item) => {
                    let inst: Vec<&str> = item.split_whitespace().collect();
                    if inst.len() > 1 {
                        match inst[0] {
                            "rm" => {
                                set.remove(&inst[1..].join(" "));
                            }
                            _ => {
                                set.insert(item.to_string());
                            }
                        }
                    } else {
                        set.insert(item);
                    }
                }
                Err(_) => {
                    println!("{},", errors::AQ);
                    continue;
                }
            },
        };
    }
}

/// Non blocking string input.
/// It sends whatever is inputed and then dies.
/// It can be "polled" with ```try_recv```
pub fn async_string(tx: Sender<String>) {
    let mut attempt = String::new();
    io::stdin().read_line(&mut attempt).expect(errors::AQ);
    tx.send(attempt).expect(errors::COM);
}

/// prints a 80 character wide string of -
pub fn bar() {
    println!("--------------------------------------------------------------------------------");
}

/// The implementor of this trait has a field that can be guessed up to three times
pub trait GuessablePassword {
    /// The field must be accessible to the trait through this method.
    fn get_password(&self) -> String;
    /// When called starts a series of interactions with the user that result
    /// in a boolean
    /// It returns `true` whenever the Password is guessed right and `false`
    /// When it has been guessed wrongly three times
    fn check_pass(&self, checks: Option<u8>) -> bool {
        let mut counter = 0;
        let checks = checks.unwrap_or(3);
        while counter < checks {
            println!("Password: ({}/{})", counter + 1, checks);
            let pass: String = get_item().unwrap();
            if pass == self.get_password() {
                break;
            }
            counter += 1;
        }
        counter < checks
    }
}
