use algebraic_traits::{Field, Invertive};

use crate::Matrix;

impl<T: Field> Matrix<T>
where
    T::Value: PartialEq + Clone,
    T::Additive: Invertive,
    T::Multiplicative: Invertive,
{
    pub fn swap_row(&mut self, i: usize, j: usize) {
        if i == j {
            return;
        }
        let w = self.w();
        for k in 0..w {
            self.val.swap(i * w + k, j * w + k);
        }
    }

    /// (掃出し後の行列, rank, det)
    pub fn row_reduction(&self) -> (Self, usize, Option<T::Value>) {
        let mut a = self.clone();
        let (rank, det) = a.row_reduction_inplace();
        (a, rank, det)
    }

    /// (rank, det)
    pub fn row_reduction_inplace(&mut self) -> (usize, Option<T::Value>) {
        if self.val.is_empty() {
            return (0, self.is_square().then_some(T::one()));
        }

        let h = self.h();
        let w = self.w();

        let mut rank = 0;
        let mut det = T::one();
        for pivot in 0..w {
            let Some(i) = (rank..h).find(|&i| self[i][pivot] != T::zero()) else {
                det = T::zero();
                continue;
            };

            if i != rank {
                self.swap_row(i, rank);
                det = T::neg(&det);
            }
            det = T::mul(&det, &self[rank][pivot]);

            let recip = T::recip(&self[rank][pivot]);
            for j in pivot..w {
                self[rank][j] = T::mul(&self[rank][j], &recip);
            }

            for i in 0..h {
                if i == rank || self[i][pivot] == T::zero() {
                    continue;
                }
                let c = T::div(&self[i][pivot], &self[rank][pivot]);
                for j in pivot..w {
                    self[i][j] = T::sub(&self[i][j], &T::mul(&c, &self[rank][j]));
                }
            }

            rank += 1;
        }

        (rank, self.is_square().then_some(det))
    }

    pub fn rank(&self) -> usize {
        let (_, rank, _) = self.row_reduction();
        rank
    }

    pub fn det(&self) -> Option<T::Value> {
        let (_, _, det) = self.row_reduction();
        det
    }

    pub fn inv(&self) -> Option<Self> {
        if !self.is_square() {
            return None;
        }
        let n = self.h();
        let mut a = Self::from_fn(n, n * 2, |i, j| {
            if j < n {
                self[i][j].clone()
            } else if j == i + n {
                T::one()
            } else {
                T::zero()
            }
        });
        a.row_reduction_inplace();
        if a[n - 1][n - 1] != T::one() {
            return None;
        }
        let inv = Self::from_fn(n, n, |i, j| a[i][j + n].clone());
        Some(inv)
    }
}
