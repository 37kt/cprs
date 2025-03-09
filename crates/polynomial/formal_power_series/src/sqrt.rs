// TODO: sparse のとき

use modint::ModInt;

use crate::{fps, FormalPowerSeries, FpsInv, FpsMul};

impl<M: ModInt<Value = u32> + FpsInv + FpsMul> FormalPowerSeries<M> {
    pub fn sqrt(&self, d: usize) -> Option<Self> {
        let n = self.len();
        let Some(l) = self
            .iter()
            .enumerate()
            .find(|&(_, x)| x.val() != 0)
            .map(|(i, _)| i) else {
                return Some(fps![0; d]);
            };
        if l & 1 != 0 {
            return None;
        }
        let y = self[l];
        let x = y.sqrt()?;
        let c = y.recip();
        let f = Self::from_fn(n - l, |i| self[l + i] * c);
        let mut f = (&sqrt_1(&f) * x).prefix(d);
        for i in (0..d).rev() {
            if i >= l / 2 {
                f[i] = f[i - l / 2];
            } else {
                f[i] = M::from_raw(0);
            }
        }
        Some(f)
    }
}

fn sqrt_1<M: ModInt<Value = u32> + FpsInv + FpsMul>(
    f: &FormalPowerSeries<M>,
) -> FormalPowerSeries<M> {
    let n = f.len();
    assert_eq!(f[0].val(), 1);
    let mut g = fps![1];
    let inv2 = M::from(2).recip();
    while g.len() < n {
        let m = n.min(g.len() * 2);
        g += &(&g.inv(m) * &f.prefix(m))[..m].into();
        g *= inv2;
    }
    g
}
