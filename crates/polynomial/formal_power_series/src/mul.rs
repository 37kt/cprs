use std::ops::{Div, DivAssign, Mul, MulAssign};

use convolution::{convolution_arbitrary_mod, convolution_ntt_friendly};
use dynamic_modint::DynamicModInt;
use modint::ModInt;
use numeric_traits::Numeric;
use static_modint::StaticModInt;

use crate::{fps, FormalPowerSeries};

impl<M: ModInt<Value = u32> + FpsMul> Mul<FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: FormalPowerSeries<M>) -> Self::Output {
        FpsMul::mul(&self, &rhs)
    }
}

impl<M: ModInt<Value = u32> + FpsMul> Mul<FormalPowerSeries<M>> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: FormalPowerSeries<M>) -> Self::Output {
        FpsMul::mul(self, &rhs)
    }
}

impl<M: ModInt<Value = u32> + FpsMul> Mul<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        FpsMul::mul(&self, rhs)
    }
}

impl<M: ModInt<Value = u32> + FpsMul> Mul<&FormalPowerSeries<M>> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: &FormalPowerSeries<M>) -> Self::Output {
        FpsMul::mul(self, rhs)
    }
}

impl<M: ModInt<Value = u32> + FpsMul> MulAssign for FormalPowerSeries<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * &rhs;
    }
}

impl<M: ModInt<Value = u32> + FpsMul> MulAssign<&FormalPowerSeries<M>> for FormalPowerSeries<M> {
    fn mul_assign(&mut self, rhs: &FormalPowerSeries<M>) {
        *self = &*self * rhs;
    }
}

impl<M: ModInt<Value = u32>> MulAssign<M> for FormalPowerSeries<M> {
    fn mul_assign(&mut self, rhs: M) {
        self.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl<M: ModInt<Value = u32>> Mul<M> for FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(mut self, rhs: M) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<M: ModInt<Value = u32>> Mul<M> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    fn mul(self, rhs: M) -> Self::Output {
        self.iter().map(|&x| x * rhs).collect()
    }
}

impl<M: ModInt<Value = u32>> DivAssign<M> for FormalPowerSeries<M> {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: M) {
        *self *= rhs.recip();
    }
}

impl<M: ModInt<Value = u32>> Div<M> for FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(mut self, rhs: M) -> Self::Output {
        self *= rhs.recip();
        self
    }
}

impl<M: ModInt<Value = u32>> Div<M> for &FormalPowerSeries<M> {
    type Output = FormalPowerSeries<M>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: M) -> Self::Output {
        self * rhs.recip()
    }
}

#[doc(hidden)]
pub trait FpsMul: ModInt<Value = u32> + Numeric {
    fn mul(f: &FormalPowerSeries<Self>, g: &FormalPowerSeries<Self>) -> FormalPowerSeries<Self>;
}

const MUL_THRESHOLD_NTT_FRIENDLY: usize = 128;
const MUL_THRESHOLD_ARBITRARY: usize = 512;

fn mul_naive<M: ModInt<Value = u32> + Numeric>(
    f: &FormalPowerSeries<M>,
    g: &FormalPowerSeries<M>,
) -> FormalPowerSeries<M> {
    if f.is_empty() || g.is_empty() {
        return fps![];
    }

    let mut h = fps![0; f.len() + g.len() - 1];
    for (i, &x) in f.iter().enumerate() {
        if x.val() == 0 {
            continue;
        }
        for (j, &y) in g.iter().enumerate() {
            h[i + j] += x * y;
        }
    }
    h
}

impl<const MOD: u32> FpsMul for StaticModInt<MOD> {
    fn mul<'a>(
        mut f: &'a FormalPowerSeries<Self>,
        mut g: &'a FormalPowerSeries<Self>,
    ) -> FormalPowerSeries<Self> {
        let mut fc = f.count_terms();
        let mut gc = g.count_terms();
        if fc > gc {
            std::mem::swap(&mut f, &mut g);
            std::mem::swap(&mut fc, &mut gc);
        }

        if StaticModInt::<MOD>::IS_NTT_FRIENDLY {
            if fc <= MUL_THRESHOLD_NTT_FRIENDLY {
                mul_naive(f, g)
            } else {
                FormalPowerSeries(convolution_ntt_friendly(f, g))
            }
        } else if fc <= MUL_THRESHOLD_ARBITRARY {
            mul_naive(f, g)
        } else {
            FormalPowerSeries(convolution_arbitrary_mod(f, g))
        }
    }
}

impl<Id> FpsMul for DynamicModInt<Id> {
    fn mul<'a>(
        mut f: &'a FormalPowerSeries<Self>,
        mut g: &'a FormalPowerSeries<Self>,
    ) -> FormalPowerSeries<Self> {
        let mut fc = f.count_terms();
        let mut gc = g.count_terms();
        if fc > gc {
            std::mem::swap(&mut f, &mut g);
            std::mem::swap(&mut fc, &mut gc);
        }

        if fc <= MUL_THRESHOLD_ARBITRARY {
            mul_naive(f, g)
        } else {
            FormalPowerSeries(convolution_arbitrary_mod(f, g))
        }
    }
}
