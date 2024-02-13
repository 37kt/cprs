use graph::Graph;

// Reference:
// https://tjkendev.github.io/procon-library/cpp/graph/auxiliary_tree.html

pub struct CompressedTree {
    fs: Vec<usize>,
    ls: Vec<usize>,
    depth: Vec<usize>,
    st: Vec<Vec<usize>>,
    lg: Vec<usize>,
    idx: Vec<usize>,
}

impl CompressedTree {
    pub fn new(g: &Graph<(), ()>) -> Self {
        let n = g.size();
        let mut c = Self {
            fs: vec![0; n],
            ls: vec![0; n],
            depth: vec![0; n],
            st: vec![vec![]],
            lg: vec![],
            idx: vec![0; n],
        };

        c.ett_dfs(0, !0, 0, g);

        c.lg = vec![0; 2];
        let m = c.st[0].len();
        for i in 2..=m {
            c.lg.push(c.lg[i >> 1] + 1);
        }

        for i in 0..c.lg[m] {
            c.st.push(vec![]);
            let b = 1 << i;
            for j in 0..=m - (b << 1) {
                let t = if c.depth[c.st[i][j]] <= c.depth[c.st[i][j + b]] {
                    c.st[i][j]
                } else {
                    c.st[i][j + b]
                };
                c.st[i + 1].push(t);
            }
        }

        c
    }

    pub fn build(&mut self, vs: &[usize]) -> (Graph<(), ()>, Vec<usize>) {
        let mut vs = vs.to_vec();
        vs.sort_by_key(|&v| self.fs[v]);
        for i in 0..vs.len() - 1 {
            vs.push(self.lca(vs[i], vs[i + 1]));
        }
        vs.sort_by_key(|&v| self.fs[v]);
        vs.dedup();
        let mut stk = vec![];
        let mut g = Graph::new(vs.len());
        let mut idx = vec![0; vs.len()];
        for i in 0..vs.len() {
            self.idx[vs[i]] = i;
            idx[i] = vs[i];
        }
        for &v in &vs {
            while stk.len() > 0 && self.ls[*stk.last().unwrap()] < self.fs[v] {
                stk.pop();
            }
            if let Some(&u) = stk.last() {
                g.add_edge(self.idx[u], self.idx[v], ());
            }
            stk.push(v);
        }
        (g, idx)
    }

    fn ett_dfs(&mut self, v: usize, p: usize, d: usize, g: &Graph<(), ()>) {
        let c = self.st[0].len();
        self.fs[v] = c;
        self.st[0].push(v);
        self.depth[v] = d;
        for &(u, _) in g.out_edges(v) {
            if u == p {
                continue;
            }
            self.ett_dfs(u, v, d + 1, g);
            self.st[0].push(v);
        }
        self.ls[v] = self.st[0].len() - 1;
    }

    fn lca(&self, u: usize, v: usize) -> usize {
        let mut x = self.fs[u];
        let mut y = self.fs[v];
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        let l = self.lg[y - x + 1];
        if self.depth[self.st[l][x]] <= self.depth[self.st[l][y + 1 - (1 << l)]] {
            self.st[l][x]
        } else {
            self.st[l][y + 1 - (1 << l)]
        }
    }
}
