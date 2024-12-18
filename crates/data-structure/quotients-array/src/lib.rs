use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use quotients::quotients;

#[derive(Clone)]
pub struct QuotientsArray<T> {
    n: usize,
    sqrt_n: usize,
    quotients: Vec<usize>,
    data: Vec<T>,
}

impl<T> QuotientsArray<T> {
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> T) -> Self {
        let sqrt_n = (n as f64).sqrt().floor() as usize;
        let data = quotients(n).map(f).collect::<Vec<_>>();
        let quotients = quotients(n).collect::<Vec<_>>();
        Self {
            n,
            sqrt_n,
            quotients,
            data,
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }

    pub fn sqrt_n(&self) -> usize {
        self.sqrt_n
    }

    pub fn quotients(&self) -> &[usize] {
        &self.quotients
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn index(&self, x: usize) -> usize {
        assert!(1 <= x && x <= self.n);
        if x <= self.sqrt_n {
            self.data.len() - x
        } else {
            self.n / x - 1
        }
    }
}

impl<T> Index<usize> for QuotientsArray<T> {
    type Output = T;
    fn index(&self, x: usize) -> &Self::Output {
        let i = self.index(x);
        unsafe { self.data.get_unchecked(i) }
    }
}

impl<T> IndexMut<usize> for QuotientsArray<T> {
    fn index_mut(&mut self, x: usize) -> &mut Self::Output {
        let i = self.index(x);
        unsafe { self.data.get_unchecked_mut(i) }
    }
}

impl<T: Debug> Debug for QuotientsArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        for i in 0..self.data.len() {
            if i > 0 {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("{}: {:?}", self.quotients[i], self.data[i]))?;
        }
        f.write_str("}")
    }
}
