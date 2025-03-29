// TODO: sparse のとき

use modint::ModInt;
use numeric_traits::Integer;

use crate::{fps, FormalPowerSeries, FpsExp, FpsInv, FpsMul};

impl<M: ModInt<Value = u32> + FpsInv + FpsMul + FpsExp> FormalPowerSeries<M> {
    pub fn pow(&self, exp: usize, d: usize) -> Self {
        if exp == 0 {
            let mut res = fps![0; d];
            res[0] = M::from(1);
            return res;
        }

        let Some(l) = self
            .iter()
            .enumerate()
            .find(|&(_, x)| x.val() != 0)
            .map(|(i, _)| i) else {
                return fps![0; d];
            };
        if l >= d.ceil_div(exp) {
            return fps![0; d];
        }

        let offset = l * exp;
        let c = self[l];
        let recip_c = c.recip();
        // let g = Self::from_fn(d - offset, |i| self[l + i] * recip_c);
        let g = Self::from_fn(d - offset, |i| {
            *self.get(l + i).unwrap_or(&0.into()) * recip_c
        });
        let mut log_g = g.log(g.len());
        log_g *= M::from(exp);
        let mut g = log_g.exp(g.len());
        g *= c.pow(exp);
        let mut res = fps![0; d];
        res[offset..].copy_from_slice(&g);
        res
    }
}
