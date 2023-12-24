use graph::Graph;

pub struct LowLink {
    vis: Vec<bool>,
    pub ord: Vec<usize>,
    pub low: Vec<usize>,
    pub par: Vec<usize>,
    pub articulation: Vec<usize>,
    pub bridge: Vec<(usize, usize)>,
    cnt: usize,
    pub component_count: usize,
}

impl LowLink {
    pub fn new(g: &Graph<(), ()>) -> Self {
        let mut this = Self {
            vis: vec![false; g.size()],
            ord: vec![0; g.size()],
            low: vec![0; g.size()],
            par: vec![!0; g.size()],
            articulation: vec![],
            bridge: vec![],
            cnt: 0,
            component_count: 0,
        };
        for i in 0..g.size() {
            if !this.vis[i] {
                this.dfs(i, !0, g);
                this.component_count += 1;
            }
        }
        this
    }

    fn dfs(&mut self, v: usize, p: usize, g: &Graph<(), ()>) {
        self.vis[v] = true;
        self.ord[v] = self.cnt;
        self.low[v] = self.cnt;
        self.par[v] = p;
        self.cnt += 1;
        let mut is_articulation = false;
        let mut cnt = 0;
        for &(u, _) in g.out_edges(v) {
            if !self.vis[u] {
                cnt += 1;
                self.dfs(u, v, g);
                if u != p {
                    self.low[v] = std::cmp::min(self.low[v], self.low[u]);
                }
                if p != !0 && self.ord[v] <= self.low[u] {
                    is_articulation = true;
                }
                if self.ord[v] < self.low[u] {
                    self.bridge.push((std::cmp::min(u, v), std::cmp::max(u, v)));
                }
            } else {
                if u != p {
                    self.low[v] = std::cmp::min(self.low[v], self.ord[u]);
                }
            }
        }
        if p == !0 && cnt > 1 {
            is_articulation = true;
        }
        if is_articulation {
            self.articulation.push(v);
        }
    }
}
