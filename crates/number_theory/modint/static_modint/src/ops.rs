use std::ops::Add;

use crate::StaticModInt;

impl<const MOD: u32> Add for StaticModInt<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut x = self.0 + rhs.0;
        if x >= MOD {
            x -= MOD;
        }
        Self(x)
    }
}
