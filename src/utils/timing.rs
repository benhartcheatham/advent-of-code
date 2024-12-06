use std::time::Instant;

pub struct Timer {
    start: Instant,
}

impl Timer {
    fn start() -> Self {
        Timer {
            start: Instant::now(),
        }
    }

    fn reset(&mut self) {
        self.start = Instant::now()
    }
}

pub fn start_benchmark(b: bool) -> Option<Timer> {
    if b {
        Some(Timer::start())
    } else {
        None
    }
}

pub fn print_time(timer: &mut Option<Timer>) {
    if let Some(timer) = timer {
        print!(" (Ran in {:.2?})", timer.start.elapsed());
        timer.reset();
    }

    println!();
}
