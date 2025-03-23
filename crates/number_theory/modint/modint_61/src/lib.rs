mod ops;
use modint::ModInt;
pub use ops::*;

mod numeric;
pub use numeric::*;

pub(crate) const P: u64 = (1 << 61) - 1;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ModInt61(u64);

impl ModInt for ModInt61 {
    type Value = u64;

    fn new<T: Into<ModInt61>>(val: T) -> Self {
        val.into()
    }

    fn modulus() -> Self::Value {
        P
    }

    fn from_raw(val: Self::Value) -> Self {
        Self::from_raw(val)
    }

    fn val(self) -> Self::Value {
        self.val()
    }

    fn recip(self) -> Self {
        self.recip()
    }

    fn pow(self, exp: usize) -> Self {
        self.pow(exp)
    }

    fn sqrt(self) -> Option<Self> {
        self.sqrt()
    }
}
