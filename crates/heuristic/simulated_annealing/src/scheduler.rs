use crate::{log_rand::LogRand, temperature::Temperature};

pub struct Scheduler<T: Temperature> {
    log_rand: LogRand,
    time_limit: f64,
    timer: std::time::Instant,
    temperature_function: T,
    progress: f64,
    temperature: f64,
}

impl<T: Temperature> Scheduler<T> {
    pub fn new(time_limit: f64, temperature_function: T) -> Self {
        let mut scheduler = Self {
            log_rand: LogRand::new(),
            time_limit,
            timer: std::time::Instant::now(),
            temperature_function,
            progress: 0.0,
            temperature: 0.0,
        };
        scheduler.update();
        scheduler
    }

    /// 温度を更新する。時間切れなら false を返す。
    pub fn update(&mut self) -> bool {
        let elapsed = self.timer.elapsed().as_secs_f64();
        if elapsed >= self.time_limit {
            return false;
        }
        self.progress = elapsed / self.time_limit;
        self.temperature = self.temperature_function.temperature(self.progress);
        true
    }

    pub fn progress(&self) -> f64 {
        self.progress
    }

    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// 大きいほど accept されやすい
    pub fn accept(&mut self, score_diff: f64) -> bool {
        score_diff > self.log_rand.next() * self.temperature
    }
}
