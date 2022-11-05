use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub trait Field:
    Sized
    + Copy
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + num_traits::Zero
    + num_traits::One
{
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Matrix<T> {
    h: usize,
    w: usize,
    v: Box<[T]>,
}

impl<T: Field> Matrix<T> {
    pub fn h(&self) -> usize {
        self.h
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            v: vec![T::zero(); h * w].into_boxed_slice(),
        }
    }

    pub fn zero(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            v: vec![T::zero(); h * w].into_boxed_slice(),
        }
    }

    pub fn e(n: usize) -> Self {
        let mut r = Self::zero(n, n);
        for i in 0..n {
            r[i][i].set_one();
        }
        r
    }

    pub fn pow(&self, mut k: usize) -> Self {
        assert_eq!(self.h, self.w);
        let mut r = Self::e(self.h);
        let mut a = self.clone();
        while k > 0 {
            if k & 1 == 1 {
                r *= &a;
            }
            a = &a * &a;
            k >>= 1;
        }
        r
    }
}

impl<T: Field> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let h = v.len();
        let w = v[0].len();
        Self {
            h,
            w,
            v: v.join(&[][..]).into_boxed_slice(),
        }
    }
}

impl<T: Field> Index<usize> for Matrix<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        let k = i * self.w;
        &self.v[k..k + self.w]
    }
}

impl<T: Field> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let k = i * self.w;
        &mut self.v[k..k + self.w]
    }
}

impl<T: Field> Add for Matrix<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] = self[i][j] + rhs[i][j];
            }
        }
        self
    }
}

impl<T: Field> AddAssign for Matrix<T> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<T: Field> Sub for Matrix<T> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] = self[i][j] - rhs[i][j];
            }
        }
        self
    }
}

impl<T: Field> SubAssign for Matrix<T> {
    fn sub_assign(&mut self, rhs: Self) {
        assert_eq!(self.h, rhs.h);
        assert_eq!(self.w, rhs.w);
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

impl<T: Field> Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        assert_eq!(self.w, rhs.h);
        let h = self.h;
        let w = rhs.w;
        let mut r = Matrix::<T>::zero(h, w);
        for i in 0..h {
            for j in 0..w {
                for k in 0..self.w {
                    r[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        r
    }
}

impl<T: Field> MulAssign<&Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Matrix<T>) {
        *self = &self.clone() * rhs
    }
}

impl<T: Field> Mul<T> for Matrix<T> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] = self[i][j] * rhs;
            }
        }
        self
    }
}

impl<T: Field> MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] *= rhs;
            }
        }
    }
}

impl<T: Field + Div<Output = T>> Div<T> for Matrix<T> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] = self[i][j] / rhs;
            }
        }
        self
    }
}

impl<T: Field + DivAssign> DivAssign<T> for Matrix<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..self.h {
            for j in 0..self.w {
                self[i][j] /= rhs;
            }
        }
    }
}
