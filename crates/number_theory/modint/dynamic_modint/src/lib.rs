use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use modint::ModInt;

mod barrett_reduction;
mod numeric;
mod ops;

pub enum DynamicModIntID {}

#[repr(transparent)]
pub struct DynamicModInt<ID>(u32, PhantomData<ID>);

pub type DefaultDynamicModInt = DynamicModInt<DynamicModIntID>;

impl<ID> Clone for DynamicModInt<ID> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData::default())
    }
}

impl<ID> Copy for DynamicModInt<ID> {}

impl<ID> Default for DynamicModInt<ID> {
    fn default() -> Self {
        Self(0, PhantomData::default())
    }
}

impl<ID> PartialEq for DynamicModInt<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<ID> Eq for DynamicModInt<ID> {}

impl<ID> Hash for DynamicModInt<ID> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<ID> ModInt for DynamicModInt<ID> {
    type Value = u32;

    fn modulus() -> Self::Value {
        barrett_reduction::barrett_reduction::<ID>().modulus()
    }

    fn from_raw(val: Self::Value) -> Self {
        Self(val, PhantomData::default())
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
