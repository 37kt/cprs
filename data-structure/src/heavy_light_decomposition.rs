use std::mem::swap;

pub struct HeavyLightDecomposition {
    t_in: Vec<usize>,
    t_out: Vec<usize>,
    size: Vec<usize>,
    heavy: Vec<usize>,
    head: Vec<usize>,
    par: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let mut hld = HeavyLightDecomposition {
            t_in: vec![0; n],
            t_out: vec![0; n],
            size: vec![0; n],
            heavy: vec![!0; n],
            head: vec![0; n],
            par: vec![!0; n],
        };
        hld.dfs_sz(g, 0);
        hld.dfs_hld(g, 0, &mut 0);
        hld
    }

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

    fn dfs_sz(&mut self, g: &Vec<Vec<usize>>, v: usize) {
        self.size[v] = 1;
        for &u in &g[v] {
            if u == self.par[v] {
                continue;
            }
            self.par[u] = v;
            self.dfs_sz(g, u);
            self.size[v] += self.size[u];
            if self.heavy[v] == !0 || self.size[u] > self.size[self.heavy[v]] {
                self.heavy[v] = u;
            }
        }
    }

    fn dfs_hld(&mut self, g: &Vec<Vec<usize>>, v: usize, t: &mut usize) {
        self.t_in[v] = *t;
        *t += 1;
        if self.heavy[v] != !0 {
            let u = self.heavy[v];
            self.head[u] = self.head[v];
            self.dfs_hld(g, u, t);
        }
        for &u in &g[v] {
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
