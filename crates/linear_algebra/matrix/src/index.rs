use std::{
    iter::FusedIterator,
    ops::{Index, IndexMut},
};

use algebraic_traits::Semiring;

use crate::Matrix;

pub struct Iter<'a, T: Semiring> {
    w: usize,
    val: &'a [T::Value],
}

impl<T: Semiring> Matrix<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            w: self.w(),
            val: &self.val,
        }
    }
}

impl<T: Semiring> Index<usize> for Matrix<T> {
    type Output = [T::Value];

    fn index(&self, index: usize) -> &Self::Output {
        &self.val[index * self.w()..(index + 1) * self.w()]
    }
}

impl<T: Semiring> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let w = self.w();
        &mut self.val[index * w..(index + 1) * w]
    }
}

impl<'a, T: Semiring> Iterator for Iter<'a, T> {
    type Item = &'a [T::Value];

    fn next(&mut self) -> Option<Self::Item> {
        if self.val.is_empty() {
            return None;
        }
        let (head, tail) = self.val.split_at(self.w);
        self.val = tail;
        Some(head)
    }
}

impl<'a, T: Semiring> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.val.is_empty() {
            return None;
        }
        let (head, tail) = self.val.split_at(self.val.len() - self.w);
        self.val = head;
        Some(tail)
    }
}

impl<'a, T: Semiring> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.val.len() / self.w
    }
}

impl<'a, T: Semiring> FusedIterator for Iter<'a, T> {}

impl<'a, T: Semiring> IntoIterator for &'a Matrix<T> {
    type Item = &'a [T::Value];
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
