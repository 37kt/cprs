use convolution::{ntt, ntt_inv};
use dynamic_modint::DynamicModInt;
use modint::ModInt;
use static_modint::StaticModInt;

use crate::mul::FpsMul;
use crate::{fps, FormalPowerSeries};

impl<M: ModInt<Value = u32> + FpsInv> FormalPowerSeries<M> {
    pub fn inv(self, d: usize) -> Self {
        FpsInv::inv(&self, d)
    }
}

#[doc(hidden)]
pub trait FpsInv: ModInt<Value = u32> {
    fn inv(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self>;
}

impl<const MOD: u32> FpsInv for StaticModInt<MOD> {
    fn inv(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self> {
        if !Self::IS_NTT_FRIENDLY {
            return inv_not_ntt_friendly(f, d);
        }

        let mut h = fps![0; d];
        h[0] = f[0].recip();
        for w in (0..).map(|i| 1 << i).take_while(|&w| w < d) {
            let mut f = f.prefix(w << 1);
            let mut g = FormalPowerSeries::from_fn(w << 1, |i| if i < w { h[i] } else { 0.into() });
            ntt(&mut f);
            ntt(&mut g);
            f.iter_mut().zip(&g).for_each(|(x, y)| *x *= *y);
            ntt_inv(&mut f);
            f.iter_mut().take(w).for_each(|x| *x = Self::from_raw(0));
            ntt(&mut f);
            f.iter_mut().zip(&g).for_each(|(x, y)| *x *= *y);
            ntt_inv(&mut f);
            h.iter_mut().zip(&f).skip(w).for_each(|(x, y)| *x = -*y);
        }
        h.truncate(d);
        h
    }
}

impl<Id> FpsInv for DynamicModInt<Id> {
    fn inv(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self> {
        inv_not_ntt_friendly(f, d)
    }
}

fn inv_not_ntt_friendly<M: ModInt<Value = u32> + FpsMul>(
    f: &FormalPowerSeries<M>,
    d: usize,
) -> FormalPowerSeries<M> {
    let mut g = fps![f[0].recip()];
    for w in (0..).map(|i| 1 << i).take_while(|&w| w < d) {
        g = &(&g + &g) - &(&(&g * &g) * &f.prefix(w << 1));
        g.truncate(w << 1);
    }
    g.truncate(d);
    g
}
