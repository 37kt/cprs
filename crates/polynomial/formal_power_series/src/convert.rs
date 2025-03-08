use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use modint::ModInt;

use crate::FormalPowerSeries;

impl<M: ModInt<Value = u32>> Deref for FormalPowerSeries<M> {
    type Target = Vec<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: ModInt<Value = u32>> DerefMut for FormalPowerSeries<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M: ModInt<Value = u32>> Debug for FormalPowerSeries<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<M: ModInt<Value = u32>> FromIterator<M> for FormalPowerSeries<M> {
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, M: ModInt<Value = u32> + 'a> FromIterator<&'a M> for FormalPowerSeries<M> {
    fn from_iter<T: IntoIterator<Item = &'a M>>(iter: T) -> Self {
        Self(iter.into_iter().copied().collect())
    }
}

impl<M: ModInt<Value = u32>> From<Vec<M>> for FormalPowerSeries<M> {
    fn from(value: Vec<M>) -> Self {
        Self(value)
    }
}

impl<M: ModInt<Value = u32>> From<&[M]> for FormalPowerSeries<M> {
    fn from(value: &[M]) -> Self {
        Self(value.to_vec())
    }
}

impl<M: ModInt<Value = u32>> IntoIterator for FormalPowerSeries<M> {
    type Item = M;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, M: ModInt<Value = u32> + 'a> IntoIterator for &'a FormalPowerSeries<M> {
    type Item = &'a M;
    type IntoIter = std::slice::Iter<'a, M>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
