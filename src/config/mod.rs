use color_eyre::{
    eyre::{Context, Result},
    Section,
};
use dialoguer::{Confirm, FuzzySelect};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt, fs, path::Path, time::Duration};
mod interact;
use history::MyHist;
mod completion;
mod history;

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Config {
    pub kill_period: Duration,
    pub work_duration: Duration,
    pub password: String,
    pub processes: HashSet<String>,
}

impl Config {
    pub fn create_at(path: &Path) -> Result<Config> {
        Confirm::new()
            .with_prompt(format!("Create path at {path:?}"))
            .interact()
            .context("failed to get a proper confirmation")?;
        let mut hist = MyHist::new();
        let mut conf = Config {
            work_duration: interact::get_work_dur(Duration::from_secs(1800))?,
            kill_period: interact::get_kill_period(Duration::from_secs(1))?,
            password: interact::get_password("a")?,
            processes: HashSet::new(),
        };
        interact::get_processes(&mut conf.processes, &mut hist)?;
        conf.write(path)?;
        Ok(conf)
    }
    fn sanity_check(mut self) -> Self {
        if self.kill_period < Duration::from_secs(1) {
            self.kill_period = Duration::from_secs(1);
        }
        self
    }
    pub fn get(filename: &Path) -> Result<Config> {
        Ok(
            serde_yaml::from_str::<Config>(&fs::read_to_string(filename)?)
                .context(format!("failed to parse {filename:?}"))
                .suggestion(format!(
                    "Delete the file and create a new one with the command
`rm {filename:?} && focus-ami --config {filename:?}`",
                ))?
                .sanity_check(),
        )
    }
    pub fn write(&self, filename: &Path) -> Result<()> {
        fs::write(
            filename,
            serde_yaml::to_string(&self).context(format!("failed to parse {self}"))?,
        )
        .context(format!("failed to write at {filename:?}"))
        .suggestion(format!(
            "Check if you have write permissions for {filename:?} and it's parent directory",
        ))
    }
    pub fn contains(&self, proc: &str) -> bool {
        self.processes.contains(proc)
    }
    pub fn usr_edit(&mut self) -> Result<()> {
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
                .context("failed to select an option")?;
            match idx {
                0 => self.work_duration = interact::get_work_dur(self.work_duration)?,
                1 => self.kill_period = interact::get_kill_period(self.kill_period)?,
                2 => self.password = interact::get_password(&self.password)?,
                3 => interact::get_processes(&mut self.processes, &mut hist)?,
                4 => break,
                _ => {}
            };
        }
        Ok(())
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
