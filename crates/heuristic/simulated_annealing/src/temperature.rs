pub trait Temperature {
    fn temperature(&self, progress: f64) -> f64;
}

pub struct Linear {
    start: f64,
    end_minus_start: f64,
}

pub struct Exponential {
    start: f64,
    end_div_start: f64,
}

impl Linear {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            end_minus_start: end - start,
        }
    }
}

impl Temperature for Linear {
    fn temperature(&self, progress: f64) -> f64 {
        self.start + self.end_minus_start * progress
    }
}

impl Exponential {
    pub fn new(start: f64, end: f64) -> Self {
        Self {
            start,
            end_div_start: end / start,
        }
    }
}

impl Temperature for Exponential {
    fn temperature(&self, progress: f64) -> f64 {
        self.start * self.end_div_start.powf(progress)
    }
}
