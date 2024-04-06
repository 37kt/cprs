use graph::Graph;

/// 強連結成分分解をし、縮約後のグラフを返す。
/// 縮約後の頂点の値には、強連結成分に属する頂点番号が格納される。
pub fn strongly_connected_components<V, E>(g: &Graph<V, E>) -> Graph<Vec<usize>, E>
where
    V: Clone,
    E: Clone,
{
    let n = g.len();
    let mut scc = Scc {
        comp: vec![0; n],
        low: vec![0; n],
        ord: vec![!0; n],
        vis: vec![],
        t: 0,
        m: 0,
    };
    for v in 0..n {
        if scc.ord[v] == !0 {
            scc.dfs(v, g);
        }
    }
    let mut groups = vec![vec![]; scc.m];
    for v in 0..n {
        scc.comp[v] = scc.m - 1 - scc.comp[v];
        groups[scc.comp[v]].push(v);
    }
    let mut edges = vec![];
    for v in 0..n {
        for (u, w) in &g[v] {
            let a = scc.comp[v];
            let b = scc.comp[*u];
            if a != b {
                edges.push((a, b, w.clone()));
            }
        }
    }
    Graph::from_vertices_and_directed_edges(&groups, &edges)
}

impl Scc {
    fn dfs<V, E>(&mut self, v: usize, g: &Graph<V, E>)
    where
        V: Clone,
        E: Clone,
    {
        self.low[v] = self.t;
        self.ord[v] = self.t;
        self.t += 1;
        self.vis.push(v);
        for &(u, _) in &g[v] {
            if self.ord[u] == !0 {
                self.dfs(u, g);
                self.low[v] = self.low[v].min(self.low[u]);
            } else {
                self.low[v] = self.low[v].min(self.ord[u]);
            }
        }
        if self.low[v] == self.ord[v] {
            loop {
                let u = self.vis.pop().unwrap();
                self.ord[u] = g.len();
                self.comp[u] = self.m;
                if u == v {
                    break;
                }
            }
            self.m += 1;
        }
    }
}

struct Scc {
    comp: Vec<usize>,
    low: Vec<usize>,
    ord: Vec<usize>,
    vis: Vec<usize>,
    t: usize,
    m: usize,
}
