use std::mem::swap;

use persistent_array::PersistentArray;

/// 完全永続 Union-Find
#[derive(Clone)]
pub struct PersistentUnionFind(PersistentArray<i32, 8>);

impl PersistentUnionFind {
    /// 辺のない Union-Find を構築する。
    pub fn new(n: usize) -> Self {
        Self(PersistentArray::from(vec![-1; n]))
    }

    /// 頂点 x が含まれる集合と頂点 y が含まれる集合を併合する。
    ///
    /// # 戻り値
    ///
    /// 併合後の Union-Find
    pub fn merge(&self, x: usize, y: usize) -> Self {
        let (mut x, mut xs) = self.leader_with_size(x);
        let (mut y, mut ys) = self.leader_with_size(y);
        if x == y {
            return self.clone();
        }
        if xs < ys {
            swap(&mut x, &mut y);
            swap(&mut xs, &mut ys);
        }
        let t = self.0.set(x, -((xs + ys) as i32));
        let t = t.set(y, x as i32);
        Self(t)
    }

    /// 頂点 x が含まれる集合のリーダーと要素数を取得する。
    ///
    /// # 戻り値
    ///
    /// - リーダー
    /// - 要素数
    pub fn leader_with_size(&self, x: usize) -> (usize, usize) {
        let t = *self.0.get(x).unwrap();
        if t < 0 {
            (x, -t as usize)
        } else {
            self.leader_with_size(t as usize)
        }
    }

    /// 頂点 x が含まれる集合のリーダーを取得する。
    pub fn leader(&self, x: usize) -> usize {
        self.leader_with_size(x).0
    }

    /// 頂点 x が含まれる集合の要素数を取得する。
    pub fn size(&self, x: usize) -> usize {
        self.leader_with_size(x).1
    }

    /// 頂点 x と頂点 y が同じ集合に含まれるかを判定する。
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader_with_size(x) == self.leader_with_size(y)
    }
}
