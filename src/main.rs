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

fn execute_during_seconds<F: Fn()>(seconds: usize, function: F) {
    for _ in 0..seconds {
        thread::sleep(ONE_SECOND);
        function()
    }
}

fn execute_during_minutes<F: Fn()>(minutes: usize, function: F) {
    let seconds = minutes * 60;
    execute_during_seconds(seconds, function);
}

fn main() {
    let x = || {
        println!("one_second");
    };
    execute_during_minutes(1, x);
}
