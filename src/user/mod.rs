use crate::{
    cli_args::ArgInfo,
    comms::{InteractToKiller, MainToFromThreads},
};
use dialoguer::{Input, Password, Select};
use std::{
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread,
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
) {
    let config = arg_info.0;
    println!("your config is: {config}");
    tx_to_killer
        .send(InteractToKiller::Config(config.clone()))
        .unwrap();

    let init_time = Instant::now();
    let mut dur = config.work_duration;
    let (tx_inst, rx_inst) = mpsc::channel();
    // let (tx_conf, rx_conf) = mpsc::channel();
    //while no new main msgs
    let mut state = InteractionStates::Start;
    while state != InteractionStates::Quit && rx_from_main.try_recv().is_err() {
        state = loop_process(
            &init_time,
            &mut dur,
            &config.password,
            state,
            &tx_inst,
            &rx_inst,
            tx_to_killer,
        );
        thread::sleep(Duration::from_millis(500));
    }
    tx_to_main.send(MainToFromThreads).unwrap();
}

fn loop_process(
    init_time: &Instant,
    dur: &mut Duration,
    passwd: &str,
    state: InteractionStates,
    tx_inst: &Sender<Instructions>,
    rx_inst: &Receiver<Instructions>,
    tx_to_killer: &Sender<InteractToKiller>,
) -> InteractionStates {
    match state {
        InteractionStates::Start => start_action(tx_inst),
        InteractionStates::Run => {
            if let Ok(instruction) = rx_inst.try_recv() {
                map_instructions(instruction, init_time, passwd, dur)
            } else {
                InteractionStates::Run
            }
        }
        InteractionStates::Pause => {
            tx_to_killer.send(InteractToKiller::Pause).unwrap();
            let tm = Instant::now();
            let _: String = Input::new()
                .with_prompt("Paused, pulse return to continue")
                .allow_empty(true)
                .interact_text()
                .unwrap();
            *dur = dur.saturating_add(tm.elapsed());
            tx_to_killer.send(InteractToKiller::Time(*dur)).unwrap();
            InteractionStates::Start
        }
        InteractionStates::Modify => {
            let mins: f64 = Input::new()
                .with_prompt("How much time should I add? [min]")
                .interact_text()
                .unwrap();
            *dur = dur.saturating_add(Duration::from_secs_f64(mins * 60.0));
            tx_to_killer.send(InteractToKiller::Time(*dur)).unwrap();
            InteractionStates::Start
        }
        InteractionStates::Quit => InteractionStates::Quit,
    }
}

fn map_instructions(
    instruction: Instructions,
    init_time: &Instant,
    passwd: &str,
    dur: &Duration,
) -> InteractionStates {
    match instruction {
        Instructions::Pause => {
            if guess_password(passwd) {
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
            if guess_password(passwd) {
                InteractionStates::Quit
            } else {
                InteractionStates::Start
            }
        }
    }
}

fn start_action(tx_inst: &Sender<Instructions>) -> InteractionStates {
    let tx_clone = tx_inst.clone();
    thread::spawn(move || {
        let idx = Select::new()
            .items(&INSTRUCTIONS)
            .with_prompt("")
            .default(0)
            .interact()
            .unwrap();
        tx_clone.send(INSTRUCTIONS[idx]).unwrap();
    });
    InteractionStates::Run
}

fn guess_password(passwd: &str) -> bool {
    let mut counter = 0;
    let checks: u8 = 3;
    while counter < 3 {
        let pass: String = Password::new()
            .with_prompt(format!("Password ({}/{})", counter + 1, checks))
            .interact()
            .unwrap();
        if pass == passwd {
            break;
        }
        counter += 1;
    }
    counter < checks
}

fn duration_to_string(t: &Duration) -> String {
    let time = t.as_secs();
    let hours = time / 3600;
    let minutes = (time - hours * 3600) / 60;
    let seconds = time - (hours * 3600 + minutes * 60);
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
