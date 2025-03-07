use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use algebraic_traits::{Invertive, Ring, Semiring};

use crate::Matrix;

impl<T: Semiring> Matrix<T>
where
    T::Value: Clone,
{
    pub fn pow(&self, mut exp: usize) -> Self {
        assert!(self.is_square());
        let mut res = Matrix::identity(self.h());
        let mut base = self.clone();
        while exp != 0 {
            if exp & 1 == 1 {
                res *= &base;
            }
            base = &base * &base;
            exp >>= 1;
        }
        res
    }
}

impl<T: Semiring> PartialEq for Matrix<T>
where
    T::Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.h() == other.h() && self.w() == other.w() && self.val == other.val
    }
}

impl<T: Semiring> Eq for Matrix<T> where T::Value: Eq {}

impl<T: Ring> Neg for Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn neg(mut self) -> Self::Output {
        for v in &mut self.val {
            *v = T::neg(v);
        }
        self
    }
}

impl<T: Ring> Neg for &Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Matrix::from_fn(self.h, self.w, |i, j| T::neg(&self[i][j]))
    }
}

impl<T: Semiring> Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += &rhs;
        self
    }
}

impl<T: Semiring> Add<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(mut self, rhs: &Matrix<T>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Semiring> Add<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, mut rhs: Matrix<T>) -> Self::Output {
        rhs += self;
        rhs
    }
}

impl<T: Semiring> Add for &Matrix<T>
where
    T::Value: Clone,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.h(), rhs.h());
        assert_eq!(self.w(), rhs.w());
        Matrix {
            h: self.h(),
            w: self.w(),
            val: self
                .val
                .iter()
                .zip(&rhs.val)
                .map(|(x, y)| T::add(x, y))
                .collect(),
        }
    }
}

impl<T: Semiring> AddAssign<&Matrix<T>> for Matrix<T> {
    fn add_assign(&mut self, rhs: &Matrix<T>) {
        assert_eq!(self.h(), rhs.h());
        assert_eq!(self.w(), rhs.w());
        self.val.iter_mut().zip(&rhs.val).for_each(|(x, y)| {
            *x = T::add(x, y);
        });
    }
}

impl<T: Semiring> AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Matrix<T>) {
        *self += &rhs;
    }
}

impl<T: Ring> Sub for Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= &rhs;
        self
    }
}

impl<T: Ring> Sub<&Matrix<T>> for Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn sub(mut self, rhs: &Matrix<T>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: Ring> Sub<Matrix<T>> for &Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn sub(self, mut rhs: Matrix<T>) -> Self::Output {
        for (x, y) in self.val.iter().zip(rhs.val.iter_mut()) {
            *y = T::sub(x, y);
        }
        rhs
    }
}

impl<T: Ring> Sub for &Matrix<T>
where
    T::Additive: Invertive,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.h(), rhs.h());
        assert_eq!(self.w(), rhs.w());
        Matrix {
            h: self.h(),
            w: self.w(),
            val: self
                .val
                .iter()
                .zip(&rhs.val)
                .map(|(x, y)| T::sub(x, y))
                .collect(),
        }
    }
}

impl<T: Ring> SubAssign<&Matrix<T>> for Matrix<T>
where
    T::Additive: Invertive,
{
    fn sub_assign(&mut self, rhs: &Matrix<T>) {
        assert_eq!(self.h(), rhs.h());
        assert_eq!(self.w(), rhs.w());
        self.val.iter_mut().zip(&rhs.val).for_each(|(x, y)| {
            *x = T::sub(x, y);
        });
    }
}

impl<T: Ring> SubAssign for Matrix<T>
where
    T::Additive: Invertive,
{
    fn sub_assign(&mut self, rhs: Matrix<T>) {
        *self -= &rhs;
    }
}

impl<T: Semiring> Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        &self * &rhs
    }
}

impl<T: Semiring> Mul<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        &self * rhs
    }
}

impl<T: Semiring> Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Semiring> Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.w(), rhs.h());
        let mut res = Matrix::<T>::zeros(self.h(), rhs.w());
        for i in 0..self.h() {
            for k in 0..self.w() {
                for j in 0..rhs.w() {
                    res[i][j] = T::add(&res[i][j], &T::mul(&self[i][k], &rhs[k][j]));
                }
            }
        }
        res
    }
}

impl<T: Semiring> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Matrix<T>) {
        *self *= &rhs;
    }
}

impl<T: Semiring> MulAssign<&Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Matrix<T>) {
        *self = &*self * rhs;
    }
}
