use rodio;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

const ONE_SECOND: Duration = Duration::from_secs(1);

struct Time {
    current: Duration,
}

trait ManageTime {
    fn new() -> Self;
    fn new_with_time(time: u64) -> Self;
    fn add_one_second(&self) -> Self;
    fn sub_one_second(&self) -> Self;
}

impl ManageTime for Time {
    fn new() -> Time {
        Time {
            current: Duration::from_secs(0),
        }
    }

    fn new_with_time(time: u64) -> Time {
        Time {
            current: Duration::from_secs(time),
        }
    }

    fn add_one_second(&self) -> Time {
        Self::new_with_time(self.current.as_secs() + 1)
    }

    fn sub_one_second(&self) -> Time {
        Self::new_with_time(self.current.as_secs() - 1)
    }
}

fn execute_during_seconds<F: Fn(Time)>(seconds: u64, function: F) {
    for time in 0..seconds {
        thread::sleep(ONE_SECOND);
        function(Time::new_with_time(time))
    }
}

fn execute_during_minutes<F: Fn(Time)>(minutes: u64, function: F) {
    let seconds = minutes * 60;
    execute_during_seconds(seconds, function);
}

fn load_audio() {
    let (_output_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open("./audios/tolling-bell_daniel-simion.mp3").unwrap();
    let source = rodio::Decoder::new_mp3(BufReader::new(file))
        .unwrap()
        .take_duration(Duration::from_secs(15));

    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.play();
    sink.sleep_until_end();
}

fn show_time(time: Time) {
    let minutes: u64 = time.current.as_secs() / 60;
    let seconds: u64 = time.current.as_secs() % 60;
    println!("{} minutes and {} seconds", minutes, seconds);
}

fn main() {
    execute_during_minutes(25, show_time);
    load_audio();
    execute_during_minutes(5, show_time);
    load_audio();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::Instant;

    fn do_nothing(_a: Time) {}

    #[test]
    fn test_execute_during_n_seconds() {
        let mut rng = rand::thread_rng();
        let time_seconds: u64 = rng.gen_range(1, 5);

        let now = Instant::now();
        execute_during_seconds(time_seconds, do_nothing);
        assert_eq!(now.elapsed().as_secs(), time_seconds);
    }
}
