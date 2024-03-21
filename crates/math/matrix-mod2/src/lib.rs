use std::{
    fmt::{Debug, Display},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, MulAssign},
};

#[derive(Clone)]
struct BitSet {
    n: usize,
    v: Vec<u64>,
}

#[derive(Clone)]
pub struct MatrixMod2 {
    n: usize,
    m: usize,
    v: Vec<BitSet>,
}

impl BitSet {
    fn new(n: usize) -> Self {
        let m = (n + 63) / 64;
        BitSet { n, v: vec![0; m] }
    }

    fn set(&mut self, i: usize, f: bool) {
        assert!(i < self.n);
        let (a, b) = (i / 64, i % 64);
        if f {
            self.v[a] |= 1 << b;
        } else {
            self.v[a] &= !(1 << b);
        }
    }

    fn get(&self, i: usize) -> bool {
        assert!(i < self.n);
        let (a, b) = (i / 64, i % 64);
        (self.v[a] >> b) & 1 == 1
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.n, other.n);
        self.v == other.v
    }
}

impl Eq for BitSet {}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        for (a, &b) in self.v.iter_mut().zip(&other.v) {
            *a |= b;
        }
    }
}

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        for (a, &b) in self.v.iter_mut().zip(&other.v) {
            *a &= b;
        }
    }
}

impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        for (a, &b) in self.v.iter_mut().zip(&other.v) {
            *a ^= b;
        }
    }
}

impl BitOr<Self> for &BitSet {
    type Output = BitSet;
    fn bitor(self, other: Self) -> BitSet {
        let mut res = self.clone();
        res |= other;
        res
    }
}

impl BitAnd<Self> for &BitSet {
    type Output = BitSet;
    fn bitand(self, other: Self) -> BitSet {
        let mut res = self.clone();
        res &= other;
        res
    }
}

impl BitXor<Self> for &BitSet {
    type Output = BitSet;
    fn bitxor(self, other: Self) -> BitSet {
        let mut res = self.clone();
        res ^= other;
        res
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            write!(f, "{}", if self.get(i) { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            write!(f, "{}", if self.get(i) { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl MatrixMod2 {
    pub fn new(n: usize, m: usize) -> Self {
        let v = vec![BitSet::new(m); n];
        MatrixMod2 { n, m, v }
    }

    pub fn set(&mut self, i: usize, j: usize, f: bool) {
        assert!(i < self.n && j < self.m);
        self.v[i].set(j, f);
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        assert!(i < self.n && j < self.m);
        self.v[i].get(j)
    }

    // (掃出し後の行列, rank)
    pub fn gauss_elimination(&self) -> (Self, usize) {
        let mut a = self.clone();
        let mut rank = 0;
        for j in 0..self.m {
            let mut pivot = None;
            for i in rank..self.n {
                if a.get(i, j) {
                    pivot = Some(i);
                    break;
                }
            }
            if let Some(pivot) = pivot {
                a.v.swap(rank, pivot);
                for i in 0..self.n {
                    if i != rank && a.get(i, j) {
                        let t = a.v[rank].clone();
                        a.v[i] ^= &t;
                    }
                }
                rank += 1;
            }
        }
        (a, rank)
    }

    pub fn det(&self) -> bool {
        assert_eq!(self.n, self.m);
        let (_, rank) = self.gauss_elimination();
        rank == self.n
    }
}

impl PartialEq for MatrixMod2 {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        self.v == other.v
    }
}

impl Eq for MatrixMod2 {}

impl BitOrAssign<&MatrixMod2> for MatrixMod2 {
    fn bitor_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        for (a, b) in self.v.iter_mut().zip(&other.v) {
            *a |= b;
        }
    }
}

impl BitAndAssign<&MatrixMod2> for MatrixMod2 {
    fn bitand_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        for (a, b) in self.v.iter_mut().zip(&other.v) {
            *a &= b;
        }
    }
}

impl BitXorAssign<&MatrixMod2> for MatrixMod2 {
    fn bitxor_assign(&mut self, other: &Self) {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        for (a, b) in self.v.iter_mut().zip(&other.v) {
            *a ^= b;
        }
    }
}

impl BitOr<&MatrixMod2> for &MatrixMod2 {
    type Output = MatrixMod2;
    fn bitor(self, other: &MatrixMod2) -> MatrixMod2 {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        let mut c = MatrixMod2::new(self.n, self.m);
        for i in 0..self.n {
            c.v[i] = &self.v[i] | &other.v[i];
        }
        c
    }
}

impl BitAnd<&MatrixMod2> for &MatrixMod2 {
    type Output = MatrixMod2;
    fn bitand(self, other: &MatrixMod2) -> MatrixMod2 {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        let mut c = MatrixMod2::new(self.n, self.m);
        for i in 0..self.n {
            c.v[i] = &self.v[i] & &other.v[i];
        }
        c
    }
}

impl BitXor<&MatrixMod2> for &MatrixMod2 {
    type Output = MatrixMod2;
    fn bitxor(self, other: &MatrixMod2) -> MatrixMod2 {
        assert_eq!(self.n, other.n);
        assert_eq!(self.m, other.m);
        let mut c = MatrixMod2::new(self.n, self.m);
        for i in 0..self.n {
            c.v[i] = &self.v[i] ^ &other.v[i];
        }
        c
    }
}

impl Mul<&MatrixMod2> for &MatrixMod2 {
    type Output = MatrixMod2;
    fn mul(self, rhs: &MatrixMod2) -> MatrixMod2 {
        assert_eq!(self.m, rhs.n);
        let mut c = MatrixMod2::new(self.n, rhs.m);
        for i in 0..self.n {
            for j in 0..self.m {
                if self.get(i, j) {
                    c.v[i] ^= &rhs.v[j];
                }
            }
        }
        c
    }
}

impl MulAssign<&MatrixMod2> for MatrixMod2 {
    fn mul_assign(&mut self, rhs: &MatrixMod2) {
        *self = &*self * rhs;
    }
}

impl Display for MatrixMod2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{}", if self.get(i, j) { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for MatrixMod2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{}", if self.get(i, j) { 1 } else { 0 })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
