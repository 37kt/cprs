use std::{
    fmt::{Debug, Write},
    ops::{Add, Index, IndexMut, Mul, Neg, Sub},
};

pub trait Element: Clone {
    type S: Copy;

    /// 0
    fn zero() -> Self::S;

    /// 1
    fn one() -> Self::S;

    /// a+b
    fn add(a: Self::S, b: Self::S) -> Self::S;

    /// a*b
    fn mul(a: Self::S, b: Self::S) -> Self::S;

    #[allow(unused_variables)]
    /// -a
    fn neg(a: Self::S) -> Self::S {
        unreachable!()
    }

    #[allow(unused_variables)]
    /// 1/a
    fn recip(a: Self::S) -> Self::S {
        unreachable!()
    }
}

#[derive(Clone)]
pub struct Matrix<E: Element> {
    h: usize,
    w: usize,
    v: Vec<E::S>,
}

impl<E: Element> Matrix<E> {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            v: vec![E::zero(); h * w],
        }
    }

    pub fn e(n: usize) -> Self {
        let mut a = Self::new(n, n);
        for i in 0..n {
            a[i][i] = E::one();
        }
        a
    }

    pub fn h(&self) -> usize {
        self.h
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn pow(&self, mut k: usize) -> Self {
        assert_eq!(self.h, self.w);
        let mut res = Self::e(self.h);
        let mut x = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                res = &res * &x;
            }
            x = &x * &x;
            k >>= 1;
        }
        res
    }
}

impl<E> Debug for Matrix<E>
where
    E: Element,
    E::S: Debug,
{
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[');
        for i in 0..self.h {
            if i > 0 {
                f.write_str(", ");
            }
            f.write_fmt(format_args!("{:?}", &self[i]));
        }
        f.write_char(']')
    }
}

impl<E: Element> Index<usize> for Matrix<E> {
    type Output = [E::S];
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index * self.w..(index + 1) * self.w]
    }
}

impl<E: Element> IndexMut<usize> for Matrix<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index * self.w..(index + 1) * self.w]
    }
}

impl<E: Element> Neg for &Matrix<E> {
    type Output = Matrix<E>;
    fn neg(self) -> Self::Output {
        Self::Output {
            h: self.h,
            w: self.w,
            v: self.v.iter().map(|&x| E::neg(x)).collect(),
        }
    }
}

impl<E: Element> Add<Self> for &Matrix<E> {
    type Output = Matrix<E>;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        let mut res = Matrix::new(self.h, self.w);
        for i in 0..self.h * self.w {
            res.v[i] = E::add(self.v[i], rhs.v[i]);
        }
        res
    }
}

impl<E: Element> Sub<Self> for &Matrix<E> {
    type Output = Matrix<E>;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        let mut res = Matrix::new(self.h, self.w);
        for i in 0..self.h * self.w {
            res.v[i] = E::add(self.v[i], E::neg(rhs.v[i]));
        }
        res
    }
}

impl<E: Element> Mul<Self> for &Matrix<E> {
    type Output = Matrix<E>;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.w, rhs.h);
        let mut res = Matrix::new(self.h, rhs.w);
        for i in 0..self.h {
            for k in 0..self.w {
                for j in 0..rhs.w {
                    res[i][j] = E::add(res[i][j], E::mul(self[i][k], rhs[k][j]));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        #[derive(Clone)]
        enum E {}
        impl Element for E {
            type S = i64;
            fn zero() -> Self::S {
                0
            }
            fn one() -> Self::S {
                1
            }
            fn add(a: Self::S, b: Self::S) -> Self::S {
                a + b
            }
            fn mul(a: Self::S, b: Self::S) -> Self::S {
                a * b
            }
        }
        let a = Matrix::<E>::e(5);
        eprintln!("{:?}", &a);
    }
}
