use graph::Graph;

/// 重心分解をする
/// 入力: 木
/// 戻り値: 重心分解の木についての (親, 行きがけ順)
pub fn build<V, E>(g: &Graph<V, E>) -> (Vec<usize>, Vec<usize>)
where
    V: Clone,
    E: Clone,
{
    let mut cd = CentroidDecomposition::new(g.len());
    cd.build(0, g);
    (cd.par, cd.ord)
}

struct CentroidDecomposition {
    sz: Vec<usize>,
    par: Vec<usize>,
    used: Vec<bool>,
    ord: Vec<usize>,
}

impl CentroidDecomposition {
    fn new(n: usize) -> Self {
        Self {
            sz: vec![1; n],
            par: vec![!0; n],
            used: vec![false; n],
            ord: vec![],
        }
    }

    fn build<V, E>(&mut self, v: usize, g: &Graph<V, E>) -> usize
    where
        V: Clone,
        E: Clone,
    {
        let sz = self.dfs_size(v, !0, g);
        let c = self.search_centroid(v, !0, sz / 2, g);
        self.used[c] = true;
        self.ord.push(v);
        for &(u, _) in &g[v] {
            if !self.used[u] {
                let d = self.build(u, g);
                self.par[d] = c;
            }
        }
        self.used[c] = false;
        c
    }

    fn dfs_size<V, E>(&mut self, v: usize, p: usize, g: &Graph<V, E>) -> usize
    where
        V: Clone,
        E: Clone,
    {
        self.sz[v] = 1;
        for &(u, _) in &g[v] {
            if u == p || self.used[u] {
                continue;
            }
            self.sz[v] += self.dfs_size(u, v, g);
        }
        self.sz[v]
    }

    fn search_centroid<V, E>(&mut self, v: usize, p: usize, mid: usize, g: &Graph<V, E>) -> usize
    where
        V: Clone,
        E: Clone,
    {
        for &(u, _) in &g[v] {
            if u == p || self.used[u] {
                continue;
            }
            if self.sz[u] > mid {
                return self.search_centroid(u, v, mid, g);
            }
        }
        v
    }
}
