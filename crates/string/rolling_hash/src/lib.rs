use std::{cell::RefCell, fmt::Debug, sync::OnceLock};

use modint_61::ModInt61;
use random::Pcg64Fast;

mod monoid;
pub use monoid::*;

mod sequence;
pub use sequence::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RollingHash {
    pub hash: ModInt61,
    pub base_pow: ModInt61, // base^len
}

impl RollingHash {
    pub fn base() -> ModInt61 {
        random::<0x_BA5E_0000>()
    }
}

impl Default for RollingHash {
    fn default() -> Self {
        Self {
            hash: ModInt61::from_raw(0),
            base_pow: ModInt61::from_raw(1),
        }
    }
}

impl Debug for RollingHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hash)
    }
}

impl<T: Into<ModInt61>> From<T> for RollingHash {
    fn from(value: T) -> Self {
        Self {
            hash: value.into() + random::<0x_ADD>(),
            base_pow: Self::base(),
        }
    }
}

impl<T> FromIterator<T> for RollingHash
where
    T: Into<ModInt61>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter()
            .fold(RollingHash::default(), |acc, x| RollingHash {
                hash: acc.hash + (x.into() + random::<0x_ADD>()) * acc.base_pow,
                base_pow: acc.base_pow * Self::base(),
            })
    }
}

impl<T> Extend<T> for RollingHash
where
    T: Into<ModInt61>,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            *self = Self {
                hash: self.hash + (x.into() + random::<0x_ADD>()) * self.base_pow,
                base_pow: self.base_pow * Self::base(),
            };
        }
    }
}

thread_local! {
    static RNG: RefCell<Pcg64Fast> = RefCell::new(Pcg64Fast::default());
}

pub(crate) fn random<const ID: u64>() -> ModInt61 {
    static VALUE: OnceLock<ModInt61> = OnceLock::new();
    *VALUE.get_or_init(|| RNG.with(|rng| ModInt61::new(rng.borrow_mut().u64())))
}
