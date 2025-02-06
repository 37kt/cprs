use libm::erf;

#[derive(Debug, Clone, Copy)]
pub struct NormalDistribution {
    mean: f64,
    std_dev: f64,
}

impl NormalDistribution {
    pub fn new(mean: f64, std_dev: f64) -> Self {
        Self { mean, std_dev }
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn std_dev(&self) -> f64 {
        self.std_dev
    }

    // x 以下の確率
    pub fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + erf((x - self.mean) / (self.std_dev * std::f64::consts::SQRT_2)))
    }

    // x 以上の確率
    pub fn ccdf(&self, x: f64) -> f64 {
        1.0 - self.cdf(x)
    }

    pub fn pdf(&self, x: f64) -> f64 {
        let d = (x - self.mean) / self.std_dev;
        (-0.5 * d * d).exp() / (self.std_dev * std::f64::consts::SQRT_2 * std::f64::consts::PI)
    }
}
