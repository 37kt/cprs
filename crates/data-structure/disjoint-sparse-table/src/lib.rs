pub trait Monoid {
    type S: Clone;
    fn e() -> Self::S;
    fn op(a: &Self::S, b: &Self::S) -> Self::S;
}

pub struct DisjointSparseTable<M>
where
    M: Monoid,
{
    t: Vec<Vec<M::S>>,
}

impl<M> DisjointSparseTable<M>
where
    M: Monoid,
{
    pub fn new(a: &[M::S]) -> Self {
        let n = a.len() + 2;
        let h = 64 - (n - 1).leading_zeros() as usize;
        let mut t = vec![vec![M::e(); n]; h];
        for k in 1..h {
            let w = 1 << k;
            for i in (w..n).step_by(w * 2) {
                for j in (i + 1 - w..i).rev() {
                    t[k][j - 1] = M::op(&a[j - 1], &t[k][j]);
                }
                let m = (i + w - 1).min(n - 1);
                for j in i..m {
                    t[k][j + 1] = M::op(&t[k][j], &a[j - 1]);
                }
            }
        }
        Self { t }
    }

    pub fn size(&self) -> usize {
        self.t[0].len() - 2
    }

    pub fn get(&self, k: usize) -> M::S {
        assert!(k < self.size());
        self.prod(k, k + 1)
    }

    pub fn prod(&self, l: usize, r: usize) -> M::S {
        assert!(l <= r && r <= self.size());
        let r = r + 1;
        let s = &self.t[63 - (l ^ r).leading_zeros() as usize];
        M::op(&s[l], &s[r])
    }
}
