use std::{cell::RefCell, mem::swap};

/// Union Find
#[derive(Clone)]
pub struct UnionFind<const UNION_BY_SIZE: bool> {
    par: RefCell<Vec<i32>>,
    cnt: usize,
}

impl<const UNION_BY_SIZE: bool> UnionFind<UNION_BY_SIZE> {
    /// 頂点 n 個と 0 本の辺で初期化する。
    pub fn new(n: usize) -> Self {
        Self {
            par: RefCell::new(vec![-1; n]),
            cnt: n,
        }
    }

    /// 頂点数を取得する。
    pub fn len(&self) -> usize {
        self.par.borrow().len()
    }

    /// 頂点 x と y を結合する。  
    /// すでに同じグループに属している場合は false を返す。
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return false;
        }
        self.cnt -= 1;
        let mut par = self.par.borrow_mut();
        if UNION_BY_SIZE && -par[x] < -par[y] {
            swap(&mut x, &mut y);
        }
        par[x] += par[y];
        par[y] = x as i32;
        true
    }

    /// 頂点 x が属する連結成分のリーダーを取得する。
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

    /// 頂点 x と y が同じ連結成分に属しているかを判定する。
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    /// 頂点 x が属する連結成分のサイズを取得する。
    pub fn size(&self, x: usize) -> usize {
        let x = self.leader(x);
        -self.par.borrow()[x] as usize
    }

    /// 連結成分の個数を取得する。
    pub fn count(&self) -> usize {
        self.cnt
    }

    /// 連結成分を取得する。
    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }
}
