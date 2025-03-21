mod constructor;
#[allow(unused_imports)]
pub use constructor::*;

mod convert;
#[allow(unused_imports)]
pub use convert::*;

mod ops;
#[allow(unused_imports)]
pub use ops::*;

mod mul;
#[allow(unused_imports)]
pub use mul::*;

mod inv;
#[allow(unused_imports)]
pub use inv::*;

mod log;
#[allow(unused_imports)]
pub use log::*;

mod exp;
#[allow(unused_imports)]
pub use exp::*;

mod pow;
#[allow(unused_imports)]
pub use pow::*;

mod sqrt;
#[allow(unused_imports)]
pub use sqrt::*;

use modint::ModInt;
use static_modint::{ModInt1000000007, ModInt998244353};

#[derive(Default, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct FormalPowerSeries<M: ModInt<Value = u32>>(pub Vec<M>);

pub type FormalPowerSeries998244353 = FormalPowerSeries<ModInt998244353>;
pub type FormalPowerSeries1000000007 = FormalPowerSeries<ModInt1000000007>;

impl<M: ModInt<Value = u32>> FormalPowerSeries<M> {
    pub fn shrink(&mut self) {
        while !self.is_empty() && self.last().unwrap().val() == 0 {
            self.pop();
        }
    }

    pub fn prefix(&self, d: usize) -> Self {
        let mut res = Vec::with_capacity(d);
        res.extend_from_slice(&self[..d.min(self.len())]);
        res.resize(d, M::from_raw(0));
        Self(res)
    }

    pub fn eval(&self, x: M) -> M {
        let mut res = M::from_raw(0);
        let mut w = M::from(1);
        for &v in self {
            res += w * v;
            w *= x;
        }
        res
    }

    pub fn count_terms(&self) -> usize {
        self.iter().filter(|&&v| v.val() != 0).count()
    }
}
