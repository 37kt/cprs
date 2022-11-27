use std::ops::{Add, Neg, Sub};

use num::Zero;

pub struct CumSum2dBuilder<T> {
    v: Vec<Vec<T>>,
}

pub struct CumSum2d<T> {
    v: Vec<Vec<T>>,
}

impl<T: Clone + Copy + Zero + Add<Output = T> + Sub<Output = T> + Neg<Output = T>>
    CumSum2dBuilder<T>
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            v: vec![vec![T::zero(); w]; h],
        }
    }

    pub fn from(v: Vec<Vec<T>>) -> Self {
        Self { v }
    }

    pub fn add(&mut self, i: usize, j: usize, x: T) {
        if i < self.v.len() && j < self.v[0].len() {
            self.v[i][j] = self.v[i][j] + x;
        }
    }

    pub fn range_add(&mut self, bi: usize, bj: usize, ei: usize, ej: usize, x: T) {
        self.add(bi, bj, x);
        self.add(ei, bj, -x);
        self.add(bi, ej, -x);
        self.add(ei, ej, x);
    }

    pub fn build(&self) -> CumSum2d<T> {
        let h = self.v.len();
        let w = self.v[0].len();
        let mut v = vec![vec![T::zero(); w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                v[i + 1][j + 1] = self.v[i][j] + v[i][j + 1] + v[i + 1][j] - v[i][j];
            }
        }
        CumSum2d { v }
    }
}

impl<T: Clone + Copy + Zero + Add<Output = T> + Sub<Output = T> + Neg<Output = T>> CumSum2d<T> {
    pub fn sum(&self, bi: usize, bj: usize, ei: usize, ej: usize) -> T {
        self.v[ei][ej] - self.v[bi][ej] - self.v[ei][bj] + self.v[bi][bj]
    }
}
