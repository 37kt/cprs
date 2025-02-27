use std::ops::Neg;

pub mod inf;
pub use inf::*;
pub mod zero_one;
pub use zero_one::*;
pub mod cast;
pub use cast::*;
pub mod numeric;
pub use numeric::*;
pub mod integer;
pub use integer::*;

pub trait Recip {
    fn recip(self) -> Self;
}

impl Recip for f32 {
    fn recip(self) -> Self {
        self.recip()
    }
}

impl Recip for f64 {
    fn recip(self) -> Self {
        self.recip()
    }
}

pub trait Signed: Sized + Neg<Output = Self> {}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(impl Signed for $t {})*
    };
}

impl_signed! {
    i8, i16, i32, i64, i128, isize, f32, f64
}
