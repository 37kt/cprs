use std::mem::swap;

use algebraic::Monoid;

/// ロールバック可能な、連結成分の総積を管理するUnionFind  
/// 可換半群を扱うが、実装の都合上 Monoid としている。
#[derive(Clone)]
pub struct RollbackUnionFindComponentSum<M, const UNION_BY_SIZE: bool = true>
where
    M: Monoid,
    M::S: Clone,
{
    par: Vec<i32>,
    sum: Vec<M::S>,
    his: Vec<(usize, i32, M::S)>,
    cnt: usize,
}

impl<M, const UNION_BY_SIZE: bool> RollbackUnionFindComponentSum<M, UNION_BY_SIZE>
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
            his: vec![],
            cnt: n,
        }
    }

    /// 頂点数を取得
    pub fn len(&self) -> usize {
        self.par.len()
    }

    /// 連結成分の数を取得
    pub fn count(&self) -> usize {
        self.cnt
    }

    /// x と y をマージする。
    /// x と y が同じ連結成分に属していない場合 true を返す。
    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        self.his.push((x, self.par[x], self.sum[x].clone()));
        self.his.push((y, self.par[y], self.sum[y].clone()));
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

    /// x が含まれる連結成分のリーダーを取得
    pub fn leader(&self, mut x: usize) -> usize {
        while self.par[x] >= 0 {
            x = self.par[x] as usize;
        }
        x
    }

    /// x と y が同じ連結成分に属しているかを判定
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    /// x が含まれる連結成分の頂点数を取得
    pub fn size(&self, x: usize) -> usize {
        -self.par[self.leader(x)] as usize
    }

    /// 直前の merge をロールバックする
    pub fn undo(&mut self) {
        for _ in 0..2 {
            let (x, par, sum) = self.his.pop().unwrap();
            if self.par[x] >= 0 && par < 0 {
                self.cnt += 1;
            }
            self.par[x] = par;
            self.sum[x] = sum;
        }
    }

    /// 連結成分のリストを取得
    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }

    /// x が含まれる連結成分の総積を取得
    pub fn sum(&self, x: usize) -> M::S {
        let x = self.leader(x);
        self.sum[x].clone()
    }
}
