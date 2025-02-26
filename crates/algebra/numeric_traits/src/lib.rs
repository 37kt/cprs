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
