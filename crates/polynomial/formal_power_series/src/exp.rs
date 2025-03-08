// TODO: sparse のとき

use convolution::{ntt, ntt_inv};
use dynamic_modint::DynamicModInt;
use modint::ModInt;
use numeric_traits::Integer;
use static_modint::StaticModInt;

use crate::{fps, FormalPowerSeries, FpsInv, FpsMul};

impl<M: ModInt<Value = u32> + FpsExp> FormalPowerSeries<M> {
    pub fn exp(&self, d: usize) -> Self {
        FpsExp::exp(self, d)
    }
}

#[doc(hidden)]
pub trait FpsExp: ModInt<Value = u32> {
    fn exp(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self>;
}

impl<const MOD: u32> FpsExp for StaticModInt<MOD> {
    fn exp(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self> {
        assert_eq!(f[0].val(), 0);

        if !Self::IS_NTT_FRIENDLY {
            return exp_not_ntt_friendly(f, d);
        }

        let mut b = fps![1, *f.get(1).unwrap_or(&0.into())];
        let mut c = fps![1];
        let mut z1: FormalPowerSeries<Self>;
        let mut z2 = fps![1, 1];
        for w in (1..).map(|i| 1 << i).take_while(|&w| w < d) {
            let mut y = b.clone();
            y.resize(w * 2, Self::from_raw(0));
            ntt(&mut y);

            z1 = z2;
            let mut z: Vec<_> = y.iter().zip(&z1).map(|(x, y)| x * y).collect();
            ntt_inv(&mut z);
            z[..w / 2].fill(Self::from_raw(0));
            ntt(&mut z);
            z.iter_mut().zip(&z1).for_each(|(x, y)| *x *= -*y);
            ntt_inv(&mut z);

            c.extend(&z[w / 2..]);
            z2 = c.clone();
            z2.resize(w * 2, Self::from_raw(0));
            ntt(&mut z2);

            let mut x = f.prefix(w).diff();
            x.push(Self::from_raw(0));
            ntt(&mut x);
            x.iter_mut().zip(&y).for_each(|(x, y)| *x *= *y);
            ntt_inv(&mut x);
            x -= b.diff();
            x.resize(w * 2, Self::from_raw(0));
            let (xl, xr) = x.split_at_mut(w);
            xl.iter_mut().zip(xr).take(w - 1).for_each(|(x, y)| {
                *y = *x;
                *x = Self::from_raw(0);
            });
            ntt(&mut x);
            x.iter_mut().zip(&z2).for_each(|(x, y)| *x *= *y);
            ntt_inv(&mut x);
            x.pop();
            x = x.integral();
            x.iter_mut().zip(f).skip(w).for_each(|(x, y)| *x += *y);
            x[..w].fill(Self::from_raw(0));
            ntt(&mut x);
            x.iter_mut().zip(&y).for_each(|(x, y)| *x *= *y);
            ntt_inv(&mut x);
            b.extend(&x[w..]);
        }

        b.truncate(d);
        b
    }
}

impl<Id> FpsExp for DynamicModInt<Id> {
    fn exp(f: &FormalPowerSeries<Self>, d: usize) -> FormalPowerSeries<Self> {
        assert_eq!(f[0].val(), 0);
        exp_not_ntt_friendly(f, d)
    }
}

fn exp_not_ntt_friendly<M: ModInt<Value = u32> + FpsMul + FpsInv>(
    f: &FormalPowerSeries<M>,
    d: usize,
) -> FormalPowerSeries<M> {
    let sz = d.ceil_pow2();
    let h = f.prefix(sz);
    let dh = h.diff();

    let mut f = fps![1];
    let mut g = fps![1];
    for w in (0..).map(|i| 1 << i).take_while(|&w| w < d) {
        let mut p = &f * &g;
        p.resize(w, M::from_raw(0));
        p *= &g;
        p.resize(w, M::from_raw(0));
        g.resize(w, M::from_raw(0));
        g.iter_mut().zip(&p).for_each(|(x, y)| *x += *x - *y);

        p = &dh[..w - 1].into() * &f;
        p.resize(w * 2 - 1, M::from_raw(0));
        p.iter_mut().for_each(|x| *x = -*x);
        p.iter_mut()
            .zip(f.iter().enumerate().skip(1))
            .for_each(|(x, (i, y))| {
                *x += *y * M::from_raw(i as u32);
            });
        p *= &g;
        p.resize(w * 2 - 1, M::from_raw(0));
        p.iter_mut()
            .zip(&dh)
            .take(w - 1)
            .for_each(|(x, y)| *x += *y);
        p = p.integral();
        p.iter_mut().zip(&h).for_each(|(x, y)| *x = *y - *x);
        p[0] += M::from_raw(1);
        f *= &p;
        f.resize(w * 2, M::from_raw(0));
    }

    f.truncate(d);
    f
}
