use crate::consts::errors;
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
    T: Fn(&mut Vec<U>) -> Option<()>,
{
    let mut vec: Vec<U> = Vec::new();
    loop {
        match function(&mut vec) {
            Some(_) =>{}
            None => break vec,
        }
    }
}


pub fn get_proc(vec: &mut Vec<String>)-> Option<()>{
    loop {
        let mut attempt = String::new();
        io::stdin().read_line(&mut attempt).expect(errors::AQ);
        match attempt.trim() {
            "\\q" => break None,
            item => match item.parse::<String>() {
                Ok(item) =>{
                    let inst: Vec<&str> = item.split_whitespace().collect();
                    if inst.len() > 1{
                        match inst[0]{
                            "rm" => {
                            }
                            a => vec.push(a.to_string()),
                        }
                    } else {
                        vec.push(item);
                    }
                },
                Err(_) => {
                    println!("{},", errors::AQ);
                    continue;
                }
            },
        }
    }
}


/// Non blocking string input
/// send whatever is inputed
/// It can be "polled" with ```try_recv```
pub fn async_string(tx: Sender<String>) {
    let mut attempt = String::new();
    io::stdin().read_line(&mut attempt).expect(errors::AQ);
    tx.send(attempt).expect(errors::COM);
}

pub fn bar() {
    println!("--------------------------------------------------------------------------------");
}

pub trait GuessablePassword {
    fn get_password(&self) -> String;
    fn check_pass(&self) -> bool {
        let mut counter = 0;
        let checks = 3;
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
