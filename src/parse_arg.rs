use clap::{App, Arg, SubCommand};

pub struct PomodoroConf {
    pub active: u64,
    pub short: u64,
    pub long: u64,
    pub frames: u64,
}

pub fn parse_arg() -> PomodoroConf {
    let matches = App::new("Rustodoro")
        .version("0.0.1")
        .about("A pomodoro in your CLI")
        .arg(
            Arg::with_name("active")
                .short("a")
                .long("active")
                .value_name("ACTIVE")
                .help("The time that you will be concentrated"),
        )
        .arg(
            Arg::with_name("short")
                .short("s")
                .long("short")
                .value_name("SHORT")
                .help("The time of a short rest"),
        )
        .arg(
            Arg::with_name("long")
                .short("l")
                .long("long")
                .value_name("LONG")
                .help("The time of a long rest"),
        )
        .get_matches();
    PomodoroConf {
        active: 0,
        short: 0,
        long: 0,
        frames: 0,
    }
}
