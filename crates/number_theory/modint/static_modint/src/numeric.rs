use numeric_traits::{Cast, Numeric, One, Recip, Zero};

use crate::StaticModInt;

impl<const MOD: u32> Zero for StaticModInt<MOD> {
    fn zero() -> Self {
        Self::from_raw(0)
    }
}

impl<const MOD: u32> One for StaticModInt<MOD> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<const MOD: u32> Numeric for StaticModInt<MOD> {}

impl<const MOD: u32> Recip for StaticModInt<MOD> {
    fn recip(self) -> Self {
        self.recip()
    }
}

macro_rules! impl_cast {
    ($($t:ty),*) => {
        $(impl<const MOD: u32> Cast<StaticModInt<MOD>> for $t {
            fn cast(self) -> StaticModInt<MOD> {
                StaticModInt::from(self)
            }
        })*
    }
}

impl_cast! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
