use std::{cell::RefCell, mem::swap};

#[derive(Clone)]
pub struct UnionFind {
    par: RefCell<Vec<i32>>,
    cnt: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: RefCell::new(vec![-1; n]),
            cnt: n,
        }
    }

    pub fn len(&self) -> usize {
        self.par.borrow().len()
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return false;
        }
        self.cnt -= 1;
        let mut par = self.par.borrow_mut();
        if -par[x] < -par[y] {
            swap(&mut x, &mut y);
        }
        par[x] += par[y];
        par[y] = x as i32;
        true
    }

    pub fn leader(&self, x: usize) -> usize {
        let mut v = x;
        let mut par = self.par.borrow_mut();
        while par[v] >= 0 {
            v = par[v] as usize;
        }
        let mut u = x;
        while par[u] >= 0 {
            let t = par[u] as usize;
            par[u] = v as i32;
            u = t;
        }
        u
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    pub fn size(&self, x: usize) -> usize {
        let x = self.leader(x);
        -self.par.borrow()[x] as usize
    }

    pub fn count(&self) -> usize {
        self.cnt
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }
}
