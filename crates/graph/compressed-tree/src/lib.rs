use graph::Graph;

// Reference:
// https://tjkendev.github.io/procon-library/cpp/graph/auxiliary_tree.html

/// 指定された頂点たちの最小共通祖先関係を保って木を圧縮してできる補助的な木
pub struct CompressedTree {
    fs: Vec<usize>,
    ls: Vec<usize>,
    depth: Vec<usize>,
    st: Vec<Vec<usize>>,
    lg: Vec<usize>,
    idx: Vec<usize>,
}

impl CompressedTree {
    /// 木 g から、圧縮木を構築する前準備をする。
    ///
    /// # 入力
    ///
    /// - `g`: 木
    ///   - ただし、頂点 0 を根とする
    ///
    /// # 計算量
    ///
    /// O(N log N)
    pub fn new(g: &Graph<(), ()>) -> Self {
        let n = g.len();
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

    /// 圧縮木を構築する。
    ///
    /// # 入力
    ///
    /// - `vs`: g の頂点の部分集合
    ///
    /// # 出力
    ///
    /// - `h`: vs に含まれる頂点同士の最小共通祖先関係を失わないように圧縮した木
    /// - `idx`: h の頂点に対応する g の頂点の index
    ///
    /// # 計算量
    ///
    /// O(|vs|)
    pub fn build(&mut self, vs: &[usize]) -> (Graph<(), ()>, Vec<usize>) {
        let mut vs = vs.to_vec();
        vs.sort_by_key(|&v| self.fs[v]);
        for i in 0..vs.len() - 1 {
            vs.push(self.lca(vs[i], vs[i + 1]));
        }
        vs.sort_by_key(|&v| self.fs[v]);
        vs.dedup();
        let mut stk = vec![];
        let mut es = vec![];
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
                es.push((self.idx[u], self.idx[v]));
            }
            stk.push(v);
        }
        (Graph::from_unweighted_directed_edges(vs.len(), &es), idx)
    }

    fn ett_dfs(&mut self, v: usize, p: usize, d: usize, g: &Graph<(), ()>) {
        let c = self.st[0].len();
        self.fs[v] = c;
        self.st[0].push(v);
        self.depth[v] = d;
        for &(u, _) in &g[v] {
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
