use csr_array::CsrArray;
use graph::{Edge, GraphBuilder, UndirectedGraph};

pub fn extended_block_cut_tree<W>(g: &CsrArray<impl Edge<W>>) -> CsrArray<usize> {
    let n = g.len();
    let mut ebct = EbctImpl {
        next: vec![!0; n],
        depth: vec![!0; n],
        add: vec![0; n],
        used: vec![false; n],
        cnt: n,
        edges: vec![],
    };

    for v in 0..n {
        if ebct.depth[v] == !0 {
            ebct.depth[v] = 0;
            ebct.dfs1(g, v);
        }
    }

    for v in 0..n {
        if ebct.depth[v] == 0 {
            ebct.dfs2(g, v, ebct.cnt);
        }
        if g[v].is_empty() {
            ebct.edges.push((v, ebct.cnt));
            ebct.cnt += 1;
        }
    }

    UndirectedGraph::from_edges(ebct.cnt, ebct.edges)
}

struct EbctImpl {
    next: Vec<usize>,
    depth: Vec<usize>,
    add: Vec<i32>,
    used: Vec<bool>,
    cnt: usize,
    edges: Vec<(usize, usize)>,
}

impl EbctImpl {
    fn dfs1<W>(&mut self, g: &CsrArray<impl Edge<W>>, v: usize) {
        for e in &g[v] {
            let u = e.to();
            if self.depth[u] == !0 {
                self.depth[u] = self.depth[v] + 1;
                self.next[v] = u;
                self.dfs1(g, u);
                self.add[v] += self.add[u];
            } else if self.depth[u] + 1 < self.depth[v] {
                self.add[v] += 1;
                self.add[self.next[u]] -= 1;
            }
        }
    }

    fn dfs2<W>(&mut self, g: &CsrArray<impl Edge<W>>, v: usize, b: usize) {
        self.used[v] = true;
        let mut ok = false;
        for e in &g[v] {
            let u = e.to();
            if self.depth[u] == self.depth[v] + 1 && !self.used[u] {
                if self.add[u] > 0 {
                    if !ok {
                        ok = true;
                        self.edges.push((v, b));
                    }
                    self.dfs2(g, u, b);
                } else {
                    self.edges.push((v, self.cnt));
                    self.cnt += 1;
                    self.dfs2(g, u, self.cnt - 1);
                }
            }
        }
        if !ok && self.depth[v] > 0 {
            self.edges.push((v, b));
        }
    }
}
