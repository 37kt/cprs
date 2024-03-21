use std::mem::swap;

use algebraic::Monoid;

#[derive(Clone)]
pub struct RollbackUnionFindComponentSum<M>
where
    M: Monoid,
    M::S: Clone,
{
    par: Vec<i32>,
    sum: Vec<M::S>,
    his: Vec<(usize, i32, M::S)>,
}

impl<M> RollbackUnionFindComponentSum<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(n: usize, a: &[M::S]) -> Self {
        Self {
            par: vec![-1; n],
            sum: a.to_vec(),
            his: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.par.len()
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        self.his.push((x, self.par[x], self.sum[x].clone()));
        self.his.push((y, self.par[y], self.sum[y].clone()));
        if x == y {
            return false;
        }
        if -self.par[x] < -self.par[y] {
            swap(&mut x, &mut y);
        }
        self.par[x] += self.par[y];
        self.sum[x] = M::op(&self.sum[x], &self.sum[y]);
        self.par[y] = x as i32;
        true
    }

    pub fn leader(&self, mut x: usize) -> usize {
        while self.par[x] >= 0 {
            x = self.par[x] as usize;
        }
        x
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    pub fn size(&self, x: usize) -> usize {
        -self.par[self.leader(x)] as usize
    }

    pub fn undo(&mut self) {
        for _ in 0..2 {
            let (x, par, sum) = self.his.pop().unwrap();
            self.par[x] = par;
            self.sum[x] = sum;
        }
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }

    pub fn sum(&self, x: usize) -> M::S {
        let x = self.leader(x);
        self.sum[x].clone()
    }
}
