use std::ops::Neg;

pub trait Signed: Sized + Neg<Output = Self> {
    fn signum(self) -> Self;
}

macro_rules! impl_signed_integer {
    ($($t:ty),*) => {
        $(impl Signed for $t {
            fn signum(self) -> Self {
                match self {
                    n if n > 0 => 1,
                    0 => 0,
                    _ => -1,
                }
            }
        })*
    };
}

macro_rules! impl_signed_float {
    ($($t:ty),*) => {
        $(impl Signed for $t {
            fn signum(self) -> Self {
                if self.is_nan() {
                    self
                } else if self == 0.0 {
                    0.0
                } else if self > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
        })*
    };
}

impl_signed_integer! {
    i8, i16, i32, i64, i128, isize
}

impl_signed_float! {
    f32, f64
}
