pub struct Timer {
    start: f64,
}

impl Timer {
    fn time_secs() -> f64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    pub fn new() -> Self {
        Self {
            start: Self::time_secs(),
        }
    }

    pub fn elapsed_secs(&self) -> f64 {
        Self::time_secs() - self.start
    }
}
