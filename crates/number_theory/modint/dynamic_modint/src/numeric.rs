use numeric_traits::{Cast, Numeric, One, Recip, Zero};

use crate::DynamicModInt;

impl<Id> Zero for DynamicModInt<Id> {
    fn zero() -> Self {
        Self::from_raw(0)
    }
}

impl<Id> One for DynamicModInt<Id> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<Id> Numeric for DynamicModInt<Id> {}

impl<Id> Recip for DynamicModInt<Id> {
    fn recip(self) -> Self {
        self.recip()
    }
}

macro_rules! impl_cast {
    ($($t:ty),*) => {
        $(impl<Id> Cast<DynamicModInt<Id>> for $t {
            fn cast(self) -> DynamicModInt<Id> {
                DynamicModInt::from(self)
            }
        })*
    }
}

impl_cast! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
