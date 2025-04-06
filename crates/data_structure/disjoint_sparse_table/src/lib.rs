use std::ops::RangeBounds;

use algebraic_traits::Monoid;
use as_half_open_range::AsHalfOpenRange;
use numeric_traits::Integer;

pub struct DisjointSparseTable<M>
where
    M: Monoid,
{
    n: usize,
    table: Vec<Vec<M::Value>>,
}

impl<M> FromIterator<M::Value> for DisjointSparseTable<M>
where
    M: Monoid,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let a = iter.into_iter().collect::<Vec<_>>();
        let n = a.len();
        let h = (n + 2).ceil_log2();
        let mut table = (0..h)
            .map(|_| (0..n + 2).map(|_| M::unit()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for (k, table) in table.iter_mut().enumerate().skip(1) {
            let w = 1 << k;
            for i in (w..n + 2).step_by(w * 2) {
                for j in (i + 1 - w..i).rev() {
                    table[j - 1] = M::op(&a[j - 1], &table[j]);
                }
                for j in i..(i + w - 1).min(n + 1) {
                    table[j + 1] = M::op(&table[j], &a[j - 1]);
                }
            }
        }
        Self { n, table }
    }
}

impl<M> DisjointSparseTable<M>
where
    M: Monoid,
{
    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn get(&self, i: usize) -> M::Value {
        self.fold(i..i + 1)
    }

    pub fn fold(&self, range: impl RangeBounds<usize>) -> M::Value {
        let (l, r) = range.as_half_open_range(0, self.n);
        let r = r + 1;
        let k = (l ^ r).floor_log2();
        let t = &self.table[k];
        M::op(&t[l], &t[r])
    }
}

impl<M> Clone for DisjointSparseTable<M>
where
    M: Monoid,
    M::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            table: self.table.clone(),
        }
    }
}
