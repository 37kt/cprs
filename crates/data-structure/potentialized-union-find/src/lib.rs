use std::{
    mem::swap,
    ops::{Add, Neg},
};

#[derive(Clone)]
pub struct PotentializedUnionFind<T, const UNION_BY_SIZE: bool = true>
where
    T: Clone + Default + Add<Output = T> + Neg<Output = T> + Eq,
{
    par: Vec<i32>,
    pot: Vec<T>,
}

impl<T, const UNION_BY_SIZE: bool> PotentializedUnionFind<T, UNION_BY_SIZE>
where
    T: Clone + Default + Add<Output = T> + Neg<Output = T> + Eq,
{
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            pot: vec![T::default(); n],
        }
    }

    pub fn len(&self) -> usize {
        self.par.len()
    }

    /// 入力: P(x) = P(y) + w
    /// 出力: 整合性があるか
    pub fn merge(&mut self, x: usize, y: usize, mut w: T) -> bool {
        w = self.potential(x) + -w + -self.potential(y);
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return w == T::default();
        }
        if UNION_BY_SIZE && -self.par[x] < -self.par[y] {
            swap(&mut x, &mut y);
            w = -w;
        }
        self.par[x] += self.par[y];
        self.par[y] = x as i32;
        self.pot[y] = w;
        true
    }

    pub fn leader(&mut self, x: usize) -> usize {
        if self.par[x] < 0 {
            x
        } else {
            let r = self.leader(self.par[x] as usize);
            self.pot[x] = self.pot[self.par[x] as usize].clone() + self.pot[x].clone();
            self.par[x] = r as i32;
            r
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.leader(x);
        -self.par[x] as usize
    }

    /// -P(leader(x)) + P(x)
    pub fn potential(&mut self, x: usize) -> T {
        self.leader(x);
        self.pot[x].clone()
    }

    /// -P(y) + P(x)
    pub fn diff(&mut self, x: usize, y: usize) -> Option<T> {
        if self.same(x, y) {
            Some(-self.potential(y) + self.potential(x))
        } else {
            None
        }
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }
}
