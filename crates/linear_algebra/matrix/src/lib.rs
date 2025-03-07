use std::fmt::Debug;

use algebraic_traits::Semiring;

pub mod index;
pub mod ops;

pub struct Matrix<T: Semiring> {
    h: usize,
    w: usize,
    val: Vec<T::Value>,
}

impl<T: Semiring> Matrix<T> {
    pub fn from_fn(h: usize, w: usize, mut f: impl FnMut(usize, usize) -> T::Value) -> Self {
        Self {
            h,
            w,
            val: (0..h * w).map(|i| f(i / w, i % w)).collect(),
        }
    }

    pub fn zeros(h: usize, w: usize) -> Self {
        Self::from_fn(h, w, |_, _| T::zero())
    }

    pub fn ones(h: usize, w: usize) -> Self {
        Self::from_fn(h, w, |_, _| T::one())
    }

    pub fn identity(n: usize) -> Self {
        Self::from_fn(n, n, |i, j| if i == j { T::one() } else { T::zero() })
    }

    pub fn h(&self) -> usize {
        self.h
    }

    pub fn w(&self) -> usize {
        self.w
    }

    pub fn is_square(&self) -> bool {
        self.h == self.w
    }
}

impl<T: Semiring> Matrix<T>
where
    T::Value: Clone,
{
    pub fn transpose(&self) -> Matrix<T> {
        Matrix::from_fn(self.w, self.h, |i, j| self.val[j * self.w + i].clone())
    }
}

impl<T: Semiring> Clone for Matrix<T>
where
    T::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            h: self.h(),
            w: self.w(),
            val: self.val.clone(),
        }
    }
}

impl<T: Semiring> Debug for Matrix<T>
where
    T::Value: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, row) in self.iter().enumerate() {
            write!(f, "{:?}", row)?;
            if i < self.h() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}
