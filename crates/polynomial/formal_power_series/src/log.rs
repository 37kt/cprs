// TODO: sparse のとき

use modint::ModInt;

use crate::{mul::FpsMul, FormalPowerSeries, FpsInv};

impl<M: ModInt<Value = u32> + FpsMul + FpsInv> FormalPowerSeries<M> {
    pub fn log(&self, d: usize) -> Self {
        assert_eq!(self[0].val(), 1);
        (&self.diff() * &self.inv(d)).prefix(d - 1).integral()
    }
}
