use std::mem::swap;

use algebraic::Monoid;

/// 連結成分の総積を管理する UnionFind  
/// 可換半群を扱うが、実装の都合上 Monoid としている。
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
    /// 頂点 i を a[i] で初期化
    pub fn new(a: &[M::S]) -> Self {
        let n = a.len();
        Self {
            par: vec![-1; n],
            sum: a.to_vec(),
            cnt: n,
        }
    }

    /// 頂点数を取得する。
    pub fn len(&self) -> usize {
        self.par.len()
    }

    /// 連結成分の個数を取得する。
    pub fn count(&self) -> usize {
        self.cnt
    }

    /// 頂点 x と y を結合する。  
    /// すでに同じ連結成分に属している場合は false を返す。
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

    /// 頂点 x が属する連結成分のリーダーを取得する。
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

    /// 頂点 x と y が同じ連結成分に属しているかを判定する。
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    /// 頂点 x が属する連結成分のサイズを取得する。
    pub fn size(&mut self, x: usize) -> usize {
        let x = self.leader(x);
        -self.par[x] as usize
    }

    /// 連結成分を取得する。
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }

    /// 頂点 x が属する連結成分の総積を取得する。
    pub fn sum(&mut self, x: usize) -> M::S {
        let x = self.leader(x);
        self.sum[x].clone()
    }
}
