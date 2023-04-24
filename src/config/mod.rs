use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, fs, path::Path, time::Duration};
use dialoguer::{Confirm, FuzzySelect};

mod interact;
use interact::MyHist;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Config {
    pub kill_period: Duration,
    pub work_duration: Duration,
    pub password: String,
    pub processes: HashSet<String>,
}

impl Config {
    pub fn create_at(path: &Path) -> Config {
        Confirm::new()
            .with_prompt("Create path at {path}")
            .interact()
            .unwrap();
        let mut hist = MyHist::new();
        let mut conf = Config {
            work_duration: interact::get_work_dur(Duration::from_secs(1800)),
            kill_period: interact::get_kill_period(Duration::from_secs(1)),
            password: interact::get_password("a"),
            processes: HashSet::new(),
        };
        interact::get_processes(&mut conf.processes, &mut hist);
        conf.write(path).unwrap();
        conf
    }
    fn sanity_check(mut self) -> Self {
        if self.kill_period < Duration::from_secs(1) {
            self.kill_period = Duration::from_secs(1);
        }
        self
    }
    pub fn get(filename: &Path) -> serde_yaml::Result<Config> {
        Ok(serde_yaml::from_str::<Config>(&fs::read_to_string(filename).unwrap())?.sanity_check())
    }
    pub fn write(&self, filename: &Path) -> std::io::Result<()> {
        fs::write(
            filename,
            serde_yaml::to_string(&self).unwrap(),
        )
    }
    /// Returns true if process is in process list, otherwise it returns false.
    pub fn contains(&self, proc: &str) -> bool {
        self.processes.contains(proc)
    }
    pub fn usr_edit(& mut self){
        const OPT_ARRAY: [&str; 5] = [
            "Default focus time",
            "Kill period",
            "Password",
            "Processes",
            "Quit",
        ];
        let mut hist = MyHist::new();
        loop {
            let idx = FuzzySelect::new()
                .with_prompt("Which field would you like to edit? (Fuzzy select)")
                .items(&OPT_ARRAY)
                .default(OPT_ARRAY.len() - 1)
                .interact()
                .unwrap();
            match idx {
                0 => self.work_duration = interact::get_work_dur(self.work_duration),
                1 => self.kill_period = interact::get_kill_period(self.kill_period),
                2 => self.password = interact::get_password(&self.password),
                3 => interact::get_processes(&mut self.processes, &mut hist),
                4 => break,
                _ => {}
            };
        };
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
kill time: {} seconds
work time: {} minutes
password: {}
processes: {:?}
",
            self.kill_period.as_secs(),
            self.work_duration.as_secs() / 60,
            self.password,
            self.processes
        )
    }
}
