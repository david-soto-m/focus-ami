use dialoguer::{History, Input};
use std::{collections::HashSet, time::Duration};

pub fn get_work_dur(orig: Duration) -> Duration {
    Duration::from_secs(loop {
        let proposed_time: u64 = Input::new()
            .with_prompt("Set the default time length [min] for focusing")
            .default(orig.as_secs() / 60)
            .interact_text()
            .unwrap();
        if let Some(tm) = proposed_time.checked_mul(60) {
            break tm;
        }
    })
}

pub fn get_kill_period(orig: Duration) -> Duration {
    Duration::from_secs(loop {
        let proposed_time: u64 = Input::new()
            .with_prompt("Set the process killing period [sec; t>0].")
            .default(orig.as_secs())
            .interact_text()
            .unwrap();
        if proposed_time > 0 {
            break proposed_time;
        }
    })
}

pub fn get_password(orig: &str) -> String {
    Input::new()
        .with_prompt("Set the password")
        .default(orig.to_string())
        .with_initial_text(orig)
        .interact()
        .unwrap()
}

enum IsEscaped {
    Escaped,
    NotEscaped,
}

fn process_rest(rest: &str) -> Vec<String> {
    let mut store: Vec<String> = vec![];
    let mut word = String::new();

    let mut is_escaped = IsEscaped::NotEscaped;
    rest.chars().for_each(|c| {
        match c {
            '\\' => match is_escaped {
                IsEscaped::Escaped => {
                    word.push(c);
                    is_escaped = IsEscaped::NotEscaped;
                }
                IsEscaped::NotEscaped => is_escaped = IsEscaped::Escaped,
            },
            ' ' => match is_escaped {
                IsEscaped::Escaped => {
                    is_escaped = IsEscaped::NotEscaped;
                    word.push(c);
                }
                IsEscaped::NotEscaped => {
                    store.push(word.clone());
                    word = String::new();
                }
            },
            _ => {
                is_escaped = IsEscaped::NotEscaped;
                word.push(c);
            }
        };
    });
    if !word.is_empty() {
        store.push(word);
    }
    store
}

fn parse_command(cmd: &str, process: &mut HashSet<String>) -> bool {
    let cmd = cmd.trim();
    if cmd.is_empty() {
        true
    } else {
        let (command, rest) = if let Some((command, rest)) = cmd.split_once(' ') {
            (command, rest)
        } else {
            (cmd, "")
        };
        let rest = process_rest(rest);
        match command.to_ascii_lowercase().as_str() {
            "add" => {
                for each in rest {
                    process.insert(each);
                }
                true
            }
            "rm" => {
                for each in rest {
                    process.remove(&each);
                }
                true
            }
            "v" | "view" => {
                println!("Current processes are: {process:#?}");
                true
            }
            "q" | "quit" => false,
            _ => true,
        }
    }
}

pub fn get_processes(process: &mut HashSet<String>, hist: &mut MyHist){
    println!(
        "
Commands:
    * add <process>
    * rm <process>
    * (q|quit)
    * (v|view)
    * diff
  Where:
    process: (proc_term|proc_term process)
    proc_term: (\\ |\\\\|[^\\ ])+
      If your process contains ' ' or '\\' you must escape the character with a
      '\\'. Escaping other characters doesn't have an effect.
      Examples:
        - The Web Content process must be written as
          `: add Web\\ Content`
        - A process called sal dn\\lkasdm
          `: add sal\\ dn\\\\lkasdm
Non ASCII characters are very badly supported (not at all)"
    );
    println!("Current processes are: {process:#?}");
    let mut cmd = String::new();
    while parse_command(&cmd, process) {
        cmd = Input::new()
            .history_with(hist)
            .with_prompt(">")
            .interact_text()
            .unwrap();
    }
}

pub struct MyHist {
    hist: Vec<String>,
}
impl MyHist {
    pub fn new() -> Self {
        Self { hist: vec![] }
    }
}

impl History<String> for MyHist {
    fn read(&self, pos: usize) -> Option<String> {
        self.hist
            .get(
                self.hist
                    .len()
                    .checked_sub(pos)
                    .and_then(|x| x.checked_sub(1))
                    .unwrap_or(0),
            )
            .cloned()
    }
    fn write(&mut self, val: &String) {
        self.hist.push(val.to_string());
    }
}
