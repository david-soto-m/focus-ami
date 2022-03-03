use crate::consts::errors;
use std::io;
use std::str::FromStr;

/// Gets [anything that implements FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html)
pub fn get_item<T>() -> Option<T>
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
