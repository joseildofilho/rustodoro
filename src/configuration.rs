use clap::{App, Arg, ArgMatches};

#[derive(Debug)]
pub struct PomodoroConf {
    pub active: u64,
    pub short: u64,
    pub long: u64,
    pub frames: u64,
}

const ACTIVE_TIME: &str = "ACTIVE";
const SHORT_TIME: &str = "SHORT";
const LONG_TIME: &str = "LONG";
const FRAMES_TIME: &str = "FRAMES";

fn get_matches<'l>() -> ArgMatches<'l> {
    App::new("Rustodoro")
        .version("0.0.1")
        .about("A pomodoro in your CLI")
        .arg(
            Arg::with_name("active")
                .short("a")
                .long("active")
                .value_name(ACTIVE_TIME)
                .help("The time that you will be concentrated"),
        )
        .arg(
            Arg::with_name("short")
                .short("s")
                .long("short")
                .value_name(SHORT_TIME)
                .help("The time of a short rest"),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .value_name(LONG_TIME)
                .help("The time of a long rest"),
        )
        .arg(
            Arg::with_name("frames")
                .short("f")
                .long("frames")
                .value_name(FRAMES_TIME)
                .help("The number of frames until a long rest"),
        )
        .get_matches()
}

fn extract_value_or_default(arg: Option<&str>, default: u64) -> u64 {
    match arg {
        Some(t) => t.parse::<u64>().unwrap(),
        None => default,
    }
}

pub fn get_configuration() -> PomodoroConf {
    let configuration = get_matches();
    PomodoroConf {
        active: extract_value_or_default(configuration.value_of(ACTIVE_TIME), 25),
        short: extract_value_or_default(configuration.value_of(SHORT_TIME), 5),
        long: extract_value_or_default(configuration.value_of(LONG_TIME), 15),
        frames: extract_value_or_default(configuration.value_of(FRAMES_TIME), 4),
    }
}
