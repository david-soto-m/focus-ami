use crate::utils;
use std::collections::HashSet;

/// Returns a valid kill time from an interaction with the user
pub fn kill_time(def: Option<u8>) -> u8 {
    println!(
        "We will set the period after which to kill processes.
Please provide a number that will be interpreted as seconds
By default: {} || Max: 255 || Min 1",
        def.unwrap_or(30)
    );
    match utils::get_item() {
        Some(num) => {
            if num > 0 {
                num
            } else {
                1
            }
        }
        None => 30,
    }
}

/// Returns a valid `work_time` from an interaction with the user
pub fn work_time(def: Option<u16>) -> u16 {
    let max_work_time = u16::MAX - u16::from(u8::MAX);

    let value = def.unwrap_or(30);
    println!(
        "Now we are going to determine the minutes you want to focus.
Please provide a number that will be interpreted as minutes
By default: {value} || Max: {max_work_time}"
    );
    let a = utils::get_item().unwrap_or(value);
    if a <= max_work_time && a >= 1 {
        a
    } else if a > max_work_time {
        max_work_time
    } else {
        1
    }
}

/// Returns a password form an interaction with the user
pub fn password(def: Option<&str>) -> String {
    println!(
        "Lets set up a password.
This is not for security, but in order to be a nuisance
That way you may be discouraged to quit the second you get distracted
By default: {} || Recommended : aslkdhgjhkadbchjqwmepam ionk",
        def.unwrap_or("a")
    );
    match utils::get_item() {
        Some(stri) => stri,
        None => def.unwrap_or("a").to_string(),
    }
}

/// Returns a `HashSet` of processes form interactions with the user, some items
/// might be bonkers but they are checked before execution
pub fn processes(def: Option<HashSet<String>>) -> HashSet<String> {
    let def = def.unwrap_or_default();
    println!(
        "Lets set up the processes to be blocked
If you need help to get the name of a process please
1. When you only have unknown processes type \\q
2. Run \"focus -a\". Alternatively use ps -e (and diff?) to figure out the name
   of your process
3. Add it to your config during a normal run
To add a process type it and press enter
To remove a process type \"rm process_name\" as in rm firefox.
To view current processes type \\w
To stop adding processes type \\q.
Current processes are:
{def:?}",
    );
    utils::get_proc(def)
}
