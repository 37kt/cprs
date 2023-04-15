use graph::Graph;
use std::mem::swap;

pub struct HeavyLightDecomposition {
    t_in: Vec<usize>,
    t_out: Vec<usize>,
    ord: Vec<usize>,
    size: Vec<usize>,
    heavy: Vec<usize>,
    head: Vec<usize>,
    par: Vec<usize>,
    depth: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new<V, E>(g: &Graph<V, E>) -> Self
    where
        V: Copy,
        E: Copy,
    {
        let n = g.size();
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
        V: Copy,
        E: Copy,
    {
        self.size[v] = 1;
        for &(u, _) in g.out_edges(v) {
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
        V: Copy,
        E: Copy,
    {
        self.t_in[v] = *t;
        self.ord.push(v);
        *t += 1;
        if self.heavy[v] != !0 {
            let u = self.heavy[v];
            self.head[u] = self.head[v];
            self.dfs_hld(g, u, t);
        }
        for &(u, _) in g.out_edges(v) {
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
    // 頂点vのk個親
    pub fn la(&self, mut v: usize, mut k: usize) -> usize {
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

    // 頂点uと頂点vのLCA
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

    // 頂点uと頂点vの距離
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let l = self.lca(u, v);
        self.depth[u] + self.depth[v] - self.depth[l] * 2
    }

    // 頂点uから頂点vにkだけ進んだときの頂点
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
            self.la(u, k)
        } else {
            self.la(v, d_lu + d_lv - k)
        }
    }

    // 頂点vのHLD上の場所
    pub fn vertex(&self, v: usize) -> usize {
        self.t_in[v]
    }

    // 辺(u,v)のHLD上の場所
    pub fn edge(&self, u: usize, v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            self.t_in[v]
        } else {
            self.t_in[u]
        }
    }

    // u->vのパスを(upパスの列,downパスの列)に分解する
    // セグ木に投げるなりして使う
    // 非可換のときはupの部分の演算を反転させる必要がある
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

    // vを根とする部分木のHLD上の範囲
    pub fn subtree(&self, v: usize, edge: bool) -> (usize, usize) {
        let e = if edge { 1 } else { 0 };
        (self.t_in[v] + e, self.t_out[v])
    }
}
