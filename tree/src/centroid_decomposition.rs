/// 重心分解をする
/// 入力: 木
/// 戻り値: 重心分解後の木?についての (親, 行きがけ順)
pub fn build(g: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let mut cd = CentroidDecomposition::new(g);
    cd.build(0);
    (cd.par, cd.ord)
}

struct CentroidDecomposition {
    g: Vec<Vec<usize>>,
    sz: Vec<usize>,
    par: Vec<usize>,
    used: Vec<bool>,
    ord: Vec<usize>,
}

impl CentroidDecomposition {
    fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        Self {
            g: g.to_vec(),
            sz: vec![1; n],
            par: vec![!0; n],
            used: vec![false; n],
            ord: vec![],
        }
    }

    fn build(&mut self, v: usize) -> usize {
        let sz = self.dfs_size(v, !0);
        let c = self.search_centroid(v, !0, sz / 2);
        self.used[c] = true;
        self.ord.push(v);
        for &u in &self.g[c].clone() {
            if !self.used[u] {
                let d = self.build(u);
                self.par[d] = c;
            }
        }
        self.used[c] = false;
        c
    }

    fn dfs_size(&mut self, v: usize, p: usize) -> usize {
        self.sz[v] = 1;
        for &u in &self.g[v].clone() {
            if u == p || self.used[u] {
                continue;
            }
            self.sz[v] += self.dfs_size(u, v);
        }
        self.sz[v]
    }

    fn search_centroid(&mut self, v: usize, p: usize, mid: usize) -> usize {
        for &u in &self.g[v] {
            if u == p || self.used[u] {
                continue;
            }
            if self.sz[u] > mid {
                return self.search_centroid(u, v, mid);
            }
        }
        v
    }
}
