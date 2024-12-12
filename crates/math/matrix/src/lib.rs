use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use algebraic::{One, Zero};

#[derive(Clone)]
pub struct Matrix<T>
where
    T: Clone,
{
    n: usize,
    m: usize,
    v: Box<[T]>,
}

impl<T> From<Vec<Vec<T>>> for Matrix<T>
where
    T: Clone,
{
    fn from(v: Vec<Vec<T>>) -> Self {
        let n = v.len();
        if n == 0 {
            return Self {
                n: 0,
                m: 0,
                v: vec![].into_boxed_slice(),
            };
        }
        let m = v[0].len();
        assert!(v.iter().all(|x| x.len() == m));
        Self {
            n,
            m,
            v: v.into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }
}

impl<T> Debug for Matrix<T>
where
    T: Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(
                    f,
                    "{:?}{}",
                    self[i][j],
                    if j + 1 == self.m {
                        if i + 1 == self.n {
                            ""
                        } else {
                            "\n"
                        }
                    } else {
                        " "
                    }
                )?
            }
        }
        Ok(())
    }
}

impl<T> Display for Matrix<T>
where
    T: Clone + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(
                    f,
                    "{}{}",
                    self[i][j],
                    if j + 1 == self.m {
                        if i + 1 == self.n {
                            ""
                        } else {
                            "\n"
                        }
                    } else {
                        " "
                    }
                )?
            }
        }
        Ok(())
    }
}

impl<T> Index<usize> for Matrix<T>
where
    T: Clone,
{
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index * self.m..(index + 1) * self.m]
    }
}

impl<T> IndexMut<usize> for Matrix<T>
where
    T: Clone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index * self.m..(index + 1) * self.m]
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn n(&self) -> usize {
        self.n
    }

    pub fn m(&self) -> usize {
        self.m
    }

    pub fn transpose(&self) -> Self {
        let mut a = Self {
            n: self.m,
            m: self.n,
            v: self.v.clone(),
        };
        for i in 0..self.n {
            for j in 0..self.m {
                a[j][i] = self[i][j].clone();
            }
        }
        a
    }

    pub fn swap_row(&mut self, i: usize, j: usize) {
        for k in 0..self.m {
            self.v.swap(i * self.m + k, j * self.m + k);
        }
    }

    pub fn swap_col(&mut self, i: usize, j: usize) {
        for k in 0..self.n {
            self.v.swap(i + k * self.m, j + k * self.m);
        }
    }
}

impl<T> Matrix<T>
where
    T: Zero + Clone,
{
    pub fn zeros(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            v: vec![T::zero(); n * m].into_boxed_slice(),
        }
    }
}

impl<T> Matrix<T>
where
    T: Zero + One + Clone,
{
    pub fn e(n: usize) -> Self {
        let mut a = Self::zeros(n, n);
        for i in 0..n {
            a[i][i] = T::one();
        }
        a
    }
}

impl<T> Matrix<T>
where
    T: Zero + One + Clone + Add<Output = T> + Mul<Output = T>,
{
    pub fn pow(&self, mut k: usize) -> Self {
        assert!(self.n == self.m);
        let mut a = self.clone();
        let mut b = Self::e(self.n);
        while k > 0 {
            if k & 1 != 0 {
                b *= &a;
            }
            a *= &a.clone();
            k >>= 1;
        }
        b
    }
}

impl<T> Matrix<T>
where
    T: Zero
        + One
        + Clone
        + Eq
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    /// (掃出し後の行列, rank, det(正方行列の場合))
    pub fn gauss_elimination(&self) -> (Self, usize, Option<T>) {
        let mut a = self.clone();
        let mut rank = 0;
        let mut det = T::one();
        let one = T::one();
        let zero = T::zero();
        for j in 0..self.m {
            if let Some(i) = (rank..self.n).position(|i| a[i][j] != zero) {
                let i = i + rank;
                if rank < i {
                    det = -det;
                    let (x, y) = a.v.split_at_mut(i * self.m);
                    x[rank * self.m..(rank + 1) * self.m].swap_with_slice(&mut y[0..self.m]);
                }
                det = det * a[rank][j].clone();
                if a[rank][j] != one {
                    let coef = one.clone() / a[rank][j].clone();
                    for k in j..self.m {
                        a[rank][k] = a[rank][k].clone() * coef.clone();
                    }
                }
                for i in 0..self.n {
                    if i == rank {
                        continue;
                    }
                    if a[i][j] != zero {
                        let coef = a[i][j].clone() / a[rank][j].clone();
                        for k in j..self.m {
                            a[i][k] = a[i][k].clone() - a[rank][k].clone() * coef.clone();
                        }
                    }
                }
                rank += 1;
            } else {
                det = zero.clone();
            }
        }
        (a, rank, Some(det))
    }

    pub fn det(&self) -> T {
        assert!(self.n == self.m);
        let (_, _, det) = self.gauss_elimination();
        det.unwrap()
    }

    pub fn inv(&self) -> Option<Self> {
        assert!(self.n == self.m);
        let one = T::one();
        let mut a = Self::zeros(self.n, self.n * 2);
        for i in 0..self.n {
            for j in 0..self.n {
                a[i][j] = self[i][j].clone();
            }
            a[i][self.n + i] = one.clone();
        }
        let (b, _, _) = a.gauss_elimination();
        if b[self.n - 1][self.n - 1] != one {
            return None;
        }
        let mut c = Self::zeros(self.n, self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                c[i][j] = b[i][self.n + j].clone();
            }
        }
        Some(c)
    }
}

impl<T> Neg for Matrix<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for x in self.v.as_mut() {
            *x = -x.clone();
        }
        self
    }
}

impl<T> Neg for &Matrix<T>
where
    T: Clone + Neg<Output = T>,
{
    type Output = Matrix<T>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

impl<T> AddAssign<&Self> for Matrix<T>
where
    T: Clone + Add<Output = T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        assert!((self.n, self.m) == (rhs.n, rhs.m));
        for (x, y) in self.v.as_mut().iter_mut().zip(rhs.v.as_ref()) {
            *x = x.clone() + y.clone();
        }
    }
}

impl<T> SubAssign<&Self> for Matrix<T>
where
    T: Clone + Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        assert!((self.n, self.m) == (rhs.n, rhs.m));
        for (x, y) in self.v.as_mut().iter_mut().zip(rhs.v.as_ref()) {
            *x = x.clone() - y.clone();
        }
    }
}

impl<T> MulAssign<&Self> for Matrix<T>
where
    T: Zero + Clone + Add<Output = T> + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: &Self) {
        assert!(self.m == rhs.n);
        let mut a = Self::zeros(self.n, rhs.m);
        for i in 0..self.n {
            for k in 0..self.m {
                for j in 0..rhs.m {
                    a[i][j] = a[i][j].clone() + self[i][k].clone() * rhs[k][j].clone();
                }
            }
        }
        *self = a;
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: Zero + Clone + Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: T) {
        for x in self.v.as_mut() {
            *x = x.clone() * rhs.clone();
        }
    }
}

impl<T> DivAssign<T> for Matrix<T>
where
    T: Zero + One + Clone + Mul<Output = T> + Div<Output = T>,
{
    fn div_assign(&mut self, rhs: T) {
        *self *= T::one() / rhs;
    }
}

impl<T> Add<Self> for &Matrix<T>
where
    T: From<i64> + Clone + Add<Output = T>,
{
    type Output = Matrix<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut a = self.clone();
        a += rhs;
        a
    }
}

impl<T> Sub<Self> for &Matrix<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut a = self.clone();
        a -= rhs;
        a
    }
}

impl<T> Mul<Self> for &Matrix<T>
where
    T: Zero + Clone + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut a = self.clone();
        a *= rhs;
        a
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Zero + Clone + Mul<Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut a = self.clone();
        a *= rhs;
        a
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Zero + One + Clone + Mul<Output = T> + Div<Output = T>,
{
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output {
        let mut a = self.clone();
        a /= rhs;
        a
    }
}
