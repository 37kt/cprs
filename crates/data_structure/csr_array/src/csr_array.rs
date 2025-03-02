use std::{iter::FusedIterator, ops::Index};

use crate::CsrArrayBuilder;

impl<T> CsrArray<T> {
    pub fn new(n: usize, idx_val: impl IntoIterator<Item = (usize, T)>) -> Self {
        let (idx, val) = idx_val.into_iter().unzip();
        let builder = CsrArrayBuilder { n, idx, val };
        builder.build()
    }

    pub fn len(&self) -> usize {
        self.sep.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.sep.len() == 1
    }

    pub fn flat_len(&self) -> usize {
        self.val.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            sep: &self.sep,
            val: &self.val,
        }
    }
}

pub struct Iter<'a, T> {
    sep: &'a [usize],
    val: &'a [T],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let &[l, r, ..] = self.sep else {
            return None;
        };
        let len = r - l;
        let (head, tail) = self.val.split_at(len);
        self.sep = &self.sep[1..];
        self.val = tail;
        Some(head)
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let &[.., l, r] = self.sep else {
            return None;
        };
        let len = r - l;
        let (head, tail) = self.val.split_at(self.val.len() - len);
        self.sep = &self.sep[..self.sep.len() - 1];
        self.val = head;
        Some(tail)
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.sep.len() - 1
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

impl<'a, T> IntoIterator for &'a CsrArray<T> {
    type Item = &'a [T];
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Index<usize> for CsrArray<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        &self.val[self.sep[index]..self.sep[index + 1]]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for CsrArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.iter().enumerate() {
            write!(f, "{:?}", v)?;
            if i < self.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[derive(Clone)]
pub struct CsrArray<T> {
    pub(crate) sep: Vec<usize>,
    pub(crate) val: Vec<T>,
}
