use timer::Timer;

const LOG_RAND_LEN: usize = 1 << 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulatedAnnealingTemperatureFunction {
    Linear,
    Exponential,
}

pub struct SimulatedAnnealingScheduler {
    start_temp: f64,
    end_temp: f64,
    temp_function: SimulatedAnnealingTemperatureFunction,
    time_limit: f64,
    timer: Timer,
    current_temp: f64,
    progress: f64,
    log_rand: Vec<f64>,
    log_rand_index: usize,
}

impl SimulatedAnnealingScheduler {
    pub fn new(
        start_temp: f64,
        end_temp: f64,
        temp_function: SimulatedAnnealingTemperatureFunction,
        time_limit: f64,
    ) -> Self {
        let timer = Timer::new();
        let mut rng = random::Pcg64Fast::default();
        let log_u64max = 2.0f64.ln() * 64.0;
        let log_rand = (0..LOG_RAND_LEN)
            .map(|_| (rng.u64() as f64).ln() - log_u64max)
            .collect();

        let mut scheduler = Self {
            start_temp,
            end_temp,
            temp_function,
            time_limit,
            timer,
            current_temp: start_temp,
            progress: 0.0,
            log_rand,
            log_rand_index: 0,
        };
        scheduler.update_temperature();
        scheduler
    }

    pub fn temperature(&self) -> f64 {
        self.current_temp
    }

    pub fn update_temperature(&mut self) -> bool {
        let elapsed = self.timer.elapsed_secs();
        if elapsed >= self.time_limit {
            return false;
        }

        self.progress = elapsed / self.time_limit;

        match self.temp_function {
            SimulatedAnnealingTemperatureFunction::Linear => {
                self.current_temp =
                    self.start_temp + (self.end_temp - self.start_temp) * self.progress;
            }
            SimulatedAnnealingTemperatureFunction::Exponential => {
                self.current_temp =
                    self.start_temp.powf(1.0 - self.progress) * self.end_temp.powf(self.progress);
            }
        }

        true
    }

    pub fn accept(&mut self, score_diff: f64) -> bool {
        self.log_rand_index = (self.log_rand_index + 1) % LOG_RAND_LEN;
        let rand = self.log_rand[self.log_rand_index];
        score_diff > self.current_temp * rand
    }
}
