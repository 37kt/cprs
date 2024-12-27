use graph::Graph;

/// 強連結成分分解をする  
///
/// # 戻り値
///
/// (groups, comp)
/// - groups: 強連結成分のグループ
/// - comp: 各頂点が属する強連結成分の番号
///
/// グループはトポロジカル順序に並んでいる
pub fn strongly_connected_components<V, E>(g: &Graph<V, E>) -> (Vec<Vec<usize>>, Vec<usize>)
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
    (groups, scc.comp)
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
