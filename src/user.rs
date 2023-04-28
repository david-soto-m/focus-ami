use crate::{
    cli_args::ArgInfo,
    comms::{InteractToKiller, MainToFromThreads},
};
use color_eyre::{
    eyre::{Context, Result},
    Report,
};
use dialoguer::{FuzzySelect, Input, Password};
use std::{
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum InteractionStates {
    Start,
    Pause,
    Run,
    Quit,
    Modify,
}

#[derive(Clone, Copy, Debug)]
enum Instructions {
    Pause,
    Quit,
    Remaining,
    Modify,
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pause => write!(f, "Pause"),
            Self::Quit => write!(f, "Quit"),
            Self::Remaining => write!(f, "Are we there yet?"),
            Self::Modify => write!(f, "Add time"),
        }
    }
}
const INSTRUCTIONS: [Instructions; 4] = [
    Instructions::Remaining,
    Instructions::Pause,
    Instructions::Modify,
    Instructions::Quit,
];

pub fn thread(
    tx_to_main: &Sender<MainToFromThreads>,
    tx_to_killer: &Sender<InteractToKiller>,
    rx_from_main: &Receiver<MainToFromThreads>,
    arg_info: ArgInfo,
) -> Result<()> {
    let config = arg_info.0;
    println!("your config is: {config}");
    tx_to_killer
        .send(InteractToKiller::Config(config.clone()))
        .context("failed to send config to killer thread")?;

    let init_time = Instant::now();
    let mut dur = config.work_duration;
    let channel_pair = mpsc::channel();
    //while no new main msgs
    let mut state = InteractionStates::Start;
    let mut inner_thread_hand = None;
    let th = loop {
        (inner_thread_hand, state) = loop_process(
            inner_thread_hand,
            &init_time,
            &mut dur,
            &config.password,
            state,
            &channel_pair,
            tx_to_killer,
        )?;
        if state != InteractionStates::Quit && rx_from_main.try_recv().is_err() {
            thread::sleep(Duration::from_millis(500));
        } else {
            break inner_thread_hand;
        }
    };
    if let Some(handle) = th {
        handle
            .join()
            .unwrap()
            .context("faild to join user input thread")?;
    }
    tx_to_main
        .send(MainToFromThreads)
        .context("failed to send kill message to main thread from user")?;
    Ok(())
}

fn loop_process(
    thread: Option<JoinHandle<Result<()>>>,
    init_time: &Instant,
    dur: &mut Duration,
    passwd: &str,
    state: InteractionStates,
    ch_pair: &(Sender<Instructions>, Receiver<Instructions>),
    tx_to_killer: &Sender<InteractToKiller>,
) -> Result<(Option<JoinHandle<Result<()>>>, InteractionStates)> {
    Ok(match state {
        InteractionStates::Start => {
            let (th, st) = start_action(&ch_pair.0);
            (Some(th), st)
        }
        InteractionStates::Run => {
            if let Ok(instruction) = &(ch_pair.1).try_recv() {
                (
                    thread,
                    map_instructions(*instruction, init_time, passwd, dur)?,
                )
            } else {
                (thread, InteractionStates::Run)
            }
        }
        InteractionStates::Pause => {
            tx_to_killer
                .send(InteractToKiller::Pause)
                .context("failed to contact Killer thread to start pause")?;
            let tm = Instant::now();
            let _: String = Input::new()
                .with_prompt("Paused, pulse return to continue")
                .allow_empty(true)
                .interact_text()
                .context("failed to get the end of the pause")?;
            *dur = dur.saturating_add(tm.elapsed());
            tx_to_killer
                .send(InteractToKiller::Time(*dur))
                .context("failed to contact Killer thread to end pause")?;
            (thread, InteractionStates::Start)
        }
        InteractionStates::Modify => {
            let mins: f64 = Input::new()
                .with_prompt("How much time should I add? [min]")
                .interact_text()
                .context("failed to get how much time to add")?;
            *dur = dur.saturating_add(Duration::from_secs_f64(mins * 60.0));
            tx_to_killer
                .send(InteractToKiller::Time(*dur))
                .context("failed to pass Killer thread the new time")?;
            (thread, InteractionStates::Start)
        }
        InteractionStates::Quit => (None, InteractionStates::Quit),
    })
}

fn map_instructions(
    instruction: Instructions,
    init_time: &Instant,
    passwd: &str,
    dur: &Duration,
) -> Result<InteractionStates> {
    Ok(match instruction {
        Instructions::Pause => {
            if guess_password(passwd)? {
                InteractionStates::Pause
            } else {
                InteractionStates::Start
            }
        }
        Instructions::Remaining => {
            let tm = init_time.elapsed();
            println!(
                "{} / {} -> {}",
                duration_to_string(&tm),
                duration_to_string(dur),
                duration_to_string(&dur.saturating_sub(tm))
            );
            InteractionStates::Start
        }
        Instructions::Modify => InteractionStates::Modify,
        Instructions::Quit => {
            if guess_password(passwd)? {
                InteractionStates::Quit
            } else {
                InteractionStates::Start
            }
        }
    })
}

fn start_action(tx_inst: &Sender<Instructions>) -> (JoinHandle<Result<()>>, InteractionStates) {
    let tx_clone = tx_inst.clone();
    let th = thread::spawn(move || {
        let idx = FuzzySelect::new()
            .items(&INSTRUCTIONS)
            .with_prompt("Select an action (Fuzzy select)")
            .default(0)
            .interact()
            .context("failed to get user instruction")?;
        tx_clone
            .send(INSTRUCTIONS[idx])
            .context("failed to send instruction to user thread")?;
        Ok::<(), Report>(())
    });
    (th, InteractionStates::Run)
}

fn guess_password(passwd: &str) -> Result<bool> {
    let mut counter = 0;
    let checks: u8 = 3;
    while counter < 3 {
        let pass: String = Password::new()
            .with_prompt(format!("Password ({}/{})", counter + 1, checks))
            .interact()
            .context("failed to get password")?;
        if pass == passwd {
            break;
        }
        counter += 1;
    }
    Ok(counter < checks)
}

fn duration_to_string(t: &Duration) -> String {
    let time = t.as_secs();
    let hours = time / 3600;
    let minutes = (time - hours * 3600) / 60;
    let seconds = time - (hours * 3600 + minutes * 60);
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
