use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use modint::ModInt;

mod barrett_reduction;
mod numeric;
mod ops;

pub enum DefaultDynamicModIntId {}

#[repr(transparent)]
pub struct DynamicModInt<Id>(u32, PhantomData<fn() -> Id>);

pub type DefaultDynamicModInt = DynamicModInt<DefaultDynamicModIntId>;

impl<Id> Clone for DynamicModInt<Id> {
    fn clone(&self) -> Self {
        Self::from_raw(self.0)
    }
}

impl<Id> Copy for DynamicModInt<Id> {}

impl<Id> Default for DynamicModInt<Id> {
    fn default() -> Self {
        Self::from_raw(0)
    }
}

impl<Id> PartialEq for DynamicModInt<Id> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Id> Eq for DynamicModInt<Id> {}

impl<Id> Hash for DynamicModInt<Id> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<Id> ModInt for DynamicModInt<Id> {
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
