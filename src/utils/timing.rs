use std::time::Instant;

pub struct Timer {
    start: Option<Instant>,
    print: bool,
}

impl Timer {
    pub fn new(print: bool) -> Self {
        Timer { start: None, print }
    }

    pub fn start(print: bool) -> Self {
        Timer {
            start: Some(Instant::now()),
            print,
        }
    }

    pub fn reset(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn time<F: FnOnce(&str)>(&mut self, f: F, arg: &str) {
        self.reset();
        f(arg);
        self.print();
    }

    pub fn print(&mut self) {
        if let Some(start) = self.start {
            if self.print {
                println!(" (Ran in {:.2?})", start.elapsed());
            }
        }
    }
}
