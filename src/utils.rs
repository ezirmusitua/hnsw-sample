use std::time::Instant;

pub struct TimeIt {
    started: Instant,
}

impl TimeIt {
    pub fn new() -> Self {
        return TimeIt {
            started: Instant::now(),
        };
    }

    pub fn print(&self, action: &str) {
        let duration = self.started.elapsed();
        println!("[INFO] {action} cost: {:?}", duration);
    }

    pub fn restart(&mut self) {
        self.started = Instant::now();
    }
}
