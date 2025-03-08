use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use modint::ModInt;

mod barrett_reduction;
mod numeric;
mod ops;

pub enum DefaultDynamicModInt64Id {}

#[repr(transparent)]
pub struct DynamicModInt64<Id>(u64, PhantomData<fn() -> Id>);

pub type DefaultDynamicModInt64 = DynamicModInt64<DefaultDynamicModInt64Id>;

impl<Id> Clone for DynamicModInt64<Id> {
    fn clone(&self) -> Self {
        Self::from_raw(self.0)
    }
}

impl<Id> Copy for DynamicModInt64<Id> {}

impl<Id> Default for DynamicModInt64<Id> {
    fn default() -> Self {
        Self::from_raw(0)
    }
}

impl<Id> PartialEq for DynamicModInt64<Id> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Id> Eq for DynamicModInt64<Id> {}

impl<Id> Hash for DynamicModInt64<Id> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<Id> ModInt for DynamicModInt64<Id> {
    type Value = u64;

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
