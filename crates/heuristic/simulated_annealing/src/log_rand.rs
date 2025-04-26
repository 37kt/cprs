const LOG_RAND_LEN: usize = 1 << 16;

thread_local! {
    static LOG_RAND: [f64; LOG_RAND_LEN] = {
        let mut rng = random::Pcg64Fast::default();
        let log_u64max = 2.0f64.ln() * 64.0;
        std::array::from_fn(|_| (rng.u64() as f64).ln() - log_u64max)
    };
}

pub(crate) struct LogRand {
    index: usize,
}

impl LogRand {
    pub fn new() -> Self {
        let mut rng = random::Pcg64Fast::default();
        Self {
            index: (rng.u64() as usize) % LOG_RAND_LEN,
        }
    }

    pub fn next(&mut self) -> f64 {
        let res = LOG_RAND.with(|log_rand| log_rand[self.index]);
        self.index += 1;
        if self.index >= LOG_RAND_LEN {
            self.index = 0;
        }
        res
    }
}
