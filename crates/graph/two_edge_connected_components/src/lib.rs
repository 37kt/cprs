use csr_array::CsrArray;
use graph::Edge;

/// 橋を除いたときの連結成分
#[derive(Clone)]
pub struct TwoEdgeConnectedComponents {
    pub comp: Vec<usize>,
    pub groups: CsrArray<usize>,
}

impl TwoEdgeConnectedComponents {
    pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {
        let mut tecc = TeccImpl {
            comp: vec![!0; g.len()],
            comp_cnt: 0,
            ord: vec![!0; g.len()],
            low: vec![!0; g.len()],
            path: vec![],
            t: 0,
        };

        for v in 0..g.len() {
            if tecc.comp[v] == !0 {
                tecc.dfs(g, v, !0);
            }
        }

        let groups = CsrArray::new(
            tecc.comp_cnt,
            tecc.comp.iter().enumerate().map(|(v, &c)| (c, v)),
        );

        Self {
            comp: tecc.comp,
            groups,
        }
    }
}

struct TeccImpl {
    comp: Vec<usize>,
    comp_cnt: usize,
    ord: Vec<usize>,
    low: Vec<usize>,
    path: Vec<usize>,
    t: usize,
}

impl TeccImpl {
    fn dfs<W>(&mut self, g: &CsrArray<impl Edge<W>>, v: usize, p: usize) {
        self.ord[v] = self.t;
        self.low[v] = self.t;
        self.t += 1;
        self.path.push(v);
        let mut f = false;
        for e in &g[v] {
            let u = e.to();
            if self.ord[u] == !0 {
                self.dfs(g, u, v);
                self.low[v] = self.low[v].min(self.low[u]);
            } else if u == p && !f {
                f = true;
            } else {
                self.low[v] = self.low[v].min(self.ord[u]);
            }
        }
        if self.ord[v] == self.low[v] {
            while self.comp[v] == !0 {
                self.comp[self.path.pop().unwrap()] = self.comp_cnt;
            }
            self.comp_cnt += 1;
        }
    }
}
