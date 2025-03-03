use csr_array::CsrArray;
use graph::Edge;

pub struct StronglyConnectedComponents {
    pub comp: Vec<usize>,
    pub groups: CsrArray<usize>,
}

impl StronglyConnectedComponents {
    pub fn new(g: &CsrArray<impl Edge>) -> Self {
        let n = g.len();
        let mut scc = SccImpl {
            comp: vec![0; n],
            low: vec![0; n],
            ord: vec![!0; n],
            path: vec![],
            t: 0,
            comp_cnt: 0,
        };

        for v in 0..n {
            if scc.ord[v] == !0 {
                scc.dfs(g, v);
            }
        }
        for v in 0..n {
            scc.comp[v] = scc.comp_cnt - 1 - scc.comp[v];
        }

        let groups = CsrArray::new(
            scc.comp_cnt,
            scc.comp.iter().enumerate().map(|(v, &c)| (c, v)),
        );

        Self {
            comp: scc.comp,
            groups,
        }
    }
}

struct SccImpl {
    comp: Vec<usize>,
    low: Vec<usize>,
    ord: Vec<usize>,
    path: Vec<usize>,
    t: usize,
    comp_cnt: usize,
}

impl SccImpl {
    fn dfs(&mut self, g: &CsrArray<impl Edge>, v: usize) {
        self.low[v] = self.t;
        self.ord[v] = self.t;
        self.t += 1;
        self.path.push(v);

        for e in &g[v] {
            let u = e.to();
            if self.ord[u] == !0 {
                self.dfs(g, u);
                self.low[v] = self.low[v].min(self.low[u]);
            } else {
                self.low[v] = self.low[v].min(self.ord[u]);
            }
        }

        if self.low[v] == self.ord[v] {
            while let Some(u) = self.path.pop() {
                self.ord[u] = !1;
                self.comp[u] = self.comp_cnt;
                if u == v {
                    break;
                }
            }
            self.comp_cnt += 1;
        }
    }
}
