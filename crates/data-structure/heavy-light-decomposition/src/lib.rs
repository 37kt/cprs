use graph::Graph;
use std::mem::swap;

/// 重軽分解
/// t_in[v]: Euler Tour における部分木 v の始点
/// t_out[v]: Euler Tour における部分木 v の終点
/// ord: Euler Tour の順序
/// size[v]: 部分木 v のサイズ
/// heavy[v]: v の heavy-edge に繋がる子
/// head[v]: v を含む heavy-path の先頭
/// par[v]: v の親
/// depth[v]: v の深さ
#[derive(Clone)]
pub struct HeavyLightDecomposition {
    pub t_in: Vec<usize>,
    pub t_out: Vec<usize>,
    pub ord: Vec<usize>,
    pub size: Vec<usize>,
    pub heavy: Vec<usize>,
    pub head: Vec<usize>,
    pub par: Vec<usize>,
    pub depth: Vec<usize>,
}

impl HeavyLightDecomposition {
    /// 重軽分解を構築する。
    ///
    /// # 入力
    ///
    /// - `g`: 連結無向グラフ
    ///
    /// # 計算量
    ///
    /// O(n)
    pub fn new<V, E>(g: &Graph<V, E>) -> Self
    where
        V: Clone,
        E: Clone,
    {
        let n = g.len();
        let mut hld = HeavyLightDecomposition {
            t_in: vec![0; n],
            t_out: vec![0; n],
            ord: vec![],
            size: vec![0; n],
            heavy: vec![!0; n],
            head: vec![0; n],
            par: vec![!0; n],
            depth: vec![0; n],
        };
        hld.dfs_sz(g, 0);
        hld.dfs_hld(g, 0, &mut 0);
        hld
    }

    fn dfs_sz<V, E>(&mut self, g: &Graph<V, E>, v: usize)
    where
        V: Clone,
        E: Clone,
    {
        self.size[v] = 1;
        for &(u, _) in &g[v] {
            if u == self.par[v] {
                continue;
            }
            self.par[u] = v;
            self.depth[u] = self.depth[v] + 1;
            self.dfs_sz(g, u);
            self.size[v] += self.size[u];
            if self.heavy[v] == !0 || self.size[u] > self.size[self.heavy[v]] {
                self.heavy[v] = u;
            }
        }
    }

    fn dfs_hld<V, E>(&mut self, g: &Graph<V, E>, v: usize, t: &mut usize)
    where
        V: Clone,
        E: Clone,
    {
        self.t_in[v] = *t;
        self.ord.push(v);
        *t += 1;
        if self.heavy[v] != !0 {
            let u = self.heavy[v];
            self.head[u] = self.head[v];
            self.dfs_hld(g, u, t);
        }
        for &(u, _) in &g[v] {
            if u == self.par[v] {
                continue;
            }
            if u == self.heavy[v] {
                continue;
            }
            self.head[u] = u;
            self.dfs_hld(g, u, t);
        }
        self.t_out[v] = *t;
    }
}

impl HeavyLightDecomposition {
    /// 頂点 v の k 個上の祖先を取得する。
    /// 存在しない場合は !0 を返す。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn kth_ancestor(&self, mut v: usize, mut k: usize) -> usize {
        if self.depth[v] < k {
            return !0;
        }
        loop {
            let u = self.head[v];
            if self.t_in[v] - k >= self.t_in[u] {
                return self.ord[self.t_in[v] - k];
            }
            k -= 1 + self.t_in[v] - self.t_in[u];
            v = self.par[u];
        }
    }

    /// 頂点 u と頂点 v の LCA を取得する。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.t_in[u] > self.t_in[v] {
                swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.par[self.head[v]];
        }
    }

    /// 頂点 u と頂点 v の距離を取得する。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let l = self.lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[l] * 2
    }

    /// 頂点 u から頂点 v に k だけ進んだときの頂点を取得する。
    /// v を超えてしまう場合は !0 を返す。
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn jump(&self, u: usize, v: usize, k: usize) -> usize {
        if k == 0 {
            return u;
        }
        let l = self.lca(u, v);
        let d_lu = self.depth[u] - self.depth[l];
        let d_lv = self.depth[v] - self.depth[l];
        if k > d_lu + d_lv {
            !0
        } else if k <= d_lu {
            self.kth_ancestor(u, k)
        } else {
            self.kth_ancestor(v, d_lu + d_lv - k)
        }
    }

    /// 頂点 v の HLD 上の場所を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn vertex(&self, v: usize) -> usize {
        self.t_in[v]
    }

    /// 辺 (u, v) の HLD 上の場所を取得する。
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn edge(&self, u: usize, v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            assert!(self.par[v] == u);
            self.t_in[v]
        } else {
            assert!(self.par[u] == v);
            self.t_in[u]
        }
    }

    /// 頂点 u から頂点 v のパスを (up パスの列, down パスの列) に分解する。  
    /// これらの列を SegmentTree 等に与えることで，
    /// パスに対するクエリを処理することができる。  
    /// 非可換の場合は up の部分の演算を反転させる必要がある。
    ///
    /// # 引数
    ///
    /// - `u`: 頂点 u
    /// - `v`: 頂点 v
    /// - `edge`: true の場合，辺クエリとして処理する。
    ///
    /// # 戻り値
    ///
    /// - `up`: up パスの列
    /// - `down`: down パスの列
    ///
    /// # 計算量
    ///
    /// O(log n)
    pub fn path(
        &self,
        mut u: usize,
        mut v: usize,
        edge: bool,
    ) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut up = vec![];
        let mut down = vec![];
        let e = if edge { 1 } else { 0 };
        while self.head[u] != self.head[v] {
            if self.t_in[u] < self.t_in[v] {
                down.push((self.t_in[self.head[v]], self.t_in[v] + 1));
                v = self.par[self.head[v]];
            } else {
                up.push((self.t_in[self.head[u]], self.t_in[u] + 1));
                u = self.par[self.head[u]];
            }
        }
        if self.t_in[u] < self.t_in[v] {
            down.push((self.t_in[u] + e, self.t_in[v] + 1));
        } else if self.t_in[u] >= self.t_in[v] + e {
            up.push((self.t_in[v] + e, self.t_in[u] + 1));
        }
        down.reverse();
        (up, down)
    }

    /// 頂点 v を根とする部分木の HLD 上の範囲を取得する。
    /// この範囲を SegmentTree 等に与えることで，
    /// 部分木に対するクエリを処理することができる。
    ///
    /// # 引数
    ///
    /// - `v`: 頂点
    /// - `edge`: true の場合，辺クエリとして処理する。
    ///
    /// # 戻り値
    ///
    /// - `l`: 部分木の HLD 上の始点
    /// - `r`: 部分木の HLD 上の終点
    ///
    /// # 計算量
    ///
    /// O(1)
    pub fn subtree(&self, v: usize, edge: bool) -> (usize, usize) {
        let e = if edge { 1 } else { 0 };
        (self.t_in[v] + e, self.t_out[v])
    }
}
