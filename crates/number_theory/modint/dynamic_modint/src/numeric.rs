use numeric_traits::{Cast, Numeric, One, Recip, Signed, Zero};

use crate::DynamicModInt;

impl<ID> Zero for DynamicModInt<ID> {
    fn zero() -> Self {
        Self::from_raw(0)
    }
}

impl<ID> One for DynamicModInt<ID> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<ID> Numeric for DynamicModInt<ID> {}

impl<ID> Recip for DynamicModInt<ID> {
    fn recip(self) -> Self {
        self.recip()
    }
}

impl<ID> Signed for DynamicModInt<ID> {}

macro_rules! impl_cast {
    ($($t:ty),*) => {
        $(impl<ID> Cast<DynamicModInt<ID>> for $t {
            fn cast(self) -> DynamicModInt<ID> {
                DynamicModInt::from(self)
            }
        })*
    }
}

impl_cast! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
