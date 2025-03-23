use numeric_traits::{Cast, Numeric, One, Recip, Zero};

use crate::ModInt61;

impl Zero for ModInt61 {
    fn zero() -> Self {
        Self::from_raw(0)
    }
}

impl One for ModInt61 {
    fn one() -> Self {
        Self::from_raw(1)
    }
}

impl Numeric for ModInt61 {}

impl Recip for ModInt61 {
    fn recip(self) -> Self {
        self.recip()
    }
}

macro_rules! impl_cast {
    ($($t:ty),*) => {
        $(impl Cast<ModInt61> for $t {
            fn cast(self) -> ModInt61 {
                ModInt61::from(self)
            }
        })*
    }
}

impl_cast! {
    u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize
}
