use modint::ModInt;

use crate::FormalPowerSeries;

/// fps![]
#[macro_export]
macro_rules! fps {
    ($($x:expr), *) => (
        $crate::FormalPowerSeries(vec![$(modint::ModInt::new($x)), *])
    );
    ($x:expr; $n:expr) => (
        $crate::FormalPowerSeries(vec![modint::ModInt::new($x); $n])
    );
}

impl<M: ModInt<Value = u32>> FormalPowerSeries<M> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M) -> Self {
        (0..n).map(f).collect()
    }
}
