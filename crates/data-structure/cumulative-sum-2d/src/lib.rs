use std::ops::{Add, Bound, Neg, RangeBounds, Sub};

pub struct CumulativeSum2D<T>
where
    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T>,
{
    v: Vec<Vec<T>>,
    built: bool,
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T>,
{
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            v: vec![vec![T::default(); w + 1]; h + 1],
            built: false,
        }
    }

    pub fn inner(&self) -> &Vec<Vec<T>> {
        &self.v
    }

    pub fn add(&mut self, i: usize, j: usize, x: T) {
        assert!(!self.built);
        if i >= self.v.len() || j >= self.v[0].len() {
            return;
        }
        self.v[i][j] = self.v[i][j].clone() + x.clone();
    }

    pub fn build(&mut self) {
        assert!(!self.built);
        for i in 1..self.v.len() {
            for j in 1..self.v[i].len() {
                self.v[i][j] =
                    self.v[i][j].clone() + self.v[i][j - 1].clone() + self.v[i - 1][j].clone()
                        - self.v[i - 1][j - 1].clone();
            }
        }
        self.built = true;
    }

    pub fn sum<R>(&self, i: R, j: R) -> T
    where
        R: RangeBounds<usize>,
    {
        assert!(self.built);
        let (bi, ei) = range_to_pair(i, self.v.len() - 1);
        let (bj, ej) = range_to_pair(j, self.v[0].len() - 1);
        self.v[ei][ej].clone() + self.v[bi][bj].clone()
            - self.v[ei][bj].clone()
            - self.v[bi][ej].clone()
    }
}

impl<T> CumulativeSum2D<T>
where
    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T> + Neg<Output = T>,
{
    pub fn imos_add<R>(&mut self, i: R, j: R, x: T)
    where
        R: RangeBounds<usize>,
    {
        assert!(!self.built);
        let (bi, ei) = range_to_pair(i, self.v.len() - 1);
        let (bj, ej) = range_to_pair(j, self.v[0].len() - 1);
        self.add(bi, bj, x.clone());
        self.add(bi, ej, -x.clone());
        self.add(ei, bj, -x.clone());
        self.add(ei, ej, x);
    }

    pub fn imos_get(&self, i: usize, j: usize) -> T {
        assert!(self.built);
        self.v[i + 1][j + 1].clone()
    }
}

impl<T> From<Vec<Vec<T>>> for CumulativeSum2D<T>
where
    T: Clone + Default + Add<T, Output = T> + Sub<T, Output = T>,
{
    fn from(mut v: Vec<Vec<T>>) -> Self {
        assert!(v.iter().all(|x| x.len() == v[0].len()));
        for x in v.iter_mut() {
            x.insert(0, T::default());
        }
        v.insert(0, vec![T::default(); v[0].len()]);
        Self { v, built: false }
    }
}

fn range_to_pair<R>(range: R, n: usize) -> (usize, usize)
where
    R: RangeBounds<usize>,
{
    let l = match range.start_bound() {
        Bound::Included(&l) => l,
        Bound::Excluded(&l) => l + 1,
        Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        Bound::Included(&r) => r + 1,
        Bound::Excluded(&r) => r,
        Bound::Unbounded => n,
    };
    (l, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut s: CumulativeSum2D<_> = a.into();
        s.build();
        eprintln!("{:?}", s.inner());
        eprintln!("{}", s.sum(1..3, 1..3));
    }
}
