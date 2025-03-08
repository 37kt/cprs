use modint::ModInt;
use ntt_precalc::NTTPrecalc;

mod mod_arithmetic;
mod ntt_precalc;
mod numeric;
mod ops;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StaticModInt<const MOD: u32>(u32);

pub type ModInt998244353 = StaticModInt<998244353>;
pub type ModInt1000000007 = StaticModInt<1000000007>;

impl<const MOD: u32> StaticModInt<MOD> {
    pub const NTT_PRECALC: NTTPrecalc = NTTPrecalc::new(MOD);
    pub const IS_PRIME: bool = Self::NTT_PRECALC.is_prime;
    pub const IS_NTT_FRIENDLY: bool = Self::NTT_PRECALC.is_prime && Self::NTT_PRECALC.rank2 >= 21;
}

impl<const MOD: u32> ModInt for StaticModInt<MOD> {
    type Value = u32;

    fn new<T: Into<Self>>(val: T) -> Self {
        val.into()
    }

    fn modulus() -> Self::Value {
        Self::modulus()
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
