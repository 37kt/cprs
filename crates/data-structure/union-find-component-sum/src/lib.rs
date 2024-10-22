use std::mem::swap;

use algebraic::Monoid;

#[derive(Clone)]
pub struct UnionFindComponentSum<M, const UNION_BY_SIZE: bool = true>
where
    M: Monoid,
    M::S: Clone,
{
    par: Vec<i32>,
    sum: Vec<M::S>,
    cnt: usize,
}

impl<M, const UNION_BY_SIZE: bool> UnionFindComponentSum<M, UNION_BY_SIZE>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(n: usize, a: &[M::S]) -> Self {
        Self {
            par: vec![-1; n],
            sum: a.to_vec(),
            cnt: n,
        }
    }

    pub fn len(&self) -> usize {
        self.par.len()
    }

    pub fn count(&self) -> usize {
        self.cnt
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return false;
        }
        self.cnt -= 1;
        if UNION_BY_SIZE && -self.par[x] < -self.par[y] {
            swap(&mut x, &mut y);
        }
        self.par[x] += self.par[y];
        self.sum[x] = M::op(&self.sum[x], &self.sum[y]);
        self.par[y] = x as i32;
        true
    }

    pub fn leader(&mut self, x: usize) -> usize {
        let mut v = x;
        while self.par[v] >= 0 {
            v = self.par[v] as usize;
        }
        let mut u = x;
        while self.par[u] >= 0 {
            let t = self.par[u] as usize;
            self.par[u] = v as i32;
            u = t;
        }
        u
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.leader(x);
        -self.par[x] as usize
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }

    pub fn sum(&mut self, x: usize) -> M::S {
        let x = self.leader(x);
        self.sum[x].clone()
    }
}
