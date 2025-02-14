use std::collections::BTreeMap;

pub trait RollbackUnionFindTrait {
    type Query: Clone;

    fn add_edge(&mut self, u: usize, v: usize);
    fn undo(&mut self);
    fn get(&mut self, query: Self::Query);
}

pub struct OfflineDynamicConnectivity<Q, UF>
where
    Q: Clone,
    UF: RollbackUnionFindTrait<Query = Q>,
{
    query_cnt: usize,
    edge_cnt: Vec<BTreeMap<usize, usize>>,
    appear: Vec<BTreeMap<usize, usize>>,
    edges: Vec<((usize, usize), (usize, usize))>,
    reads: Vec<(usize, Q)>,
    ri: usize,
    segsz: usize,
    uf: UF,
}

impl<Q, UF> OfflineDynamicConnectivity<Q, UF>
where
    Q: Clone,
    UF: RollbackUnionFindTrait<Query = Q>,
{
    pub fn new(uf: UF) -> Self {
        Self {
            query_cnt: 0,
            edge_cnt: vec![],
            appear: vec![],
            edges: vec![],
            reads: vec![],
            ri: 0,
            segsz: 0,
            uf,
        }
    }

    pub fn add_edge(&mut self, x: usize, y: usize) {
        let e = (x.min(y), x.max(y));
        while self.edge_cnt.len() <= e.0 {
            self.edge_cnt.push(BTreeMap::new());
            self.appear.push(BTreeMap::new());
        }
        if *self.edge_cnt[e.0]
            .entry(e.1)
            .and_modify(|e| *e += 1)
            .or_insert(1)
            == 1
        {
            self.appear[e.0].insert(e.1, self.query_cnt);
        }
    }

    pub fn remove_edge(&mut self, x: usize, y: usize) {
        let e = (x.min(y), x.max(y));
        let c = self.edge_cnt[e.0].get_mut(&e.1).unwrap();
        *c -= 1;
        if *c == 0 {
            let a = self.appear[e.0][&e.1];
            if a < self.query_cnt {
                self.edges.push((e, (a, self.query_cnt)));
            }
            self.edge_cnt[e.0].remove(&e.1);
            self.appear[e.0].remove(&e.1);
        }
    }

    pub fn get(&mut self, query: Q) {
        self.reads.push((self.query_cnt, query));
        self.query_cnt += 1;
    }

    pub fn run(&mut self) {
        for x in 0..self.edge_cnt.len() {
            for (&y, &c) in &self.edge_cnt[x] {
                if c > 0 {
                    self.edges
                        .push(((x, y), (self.appear[x][&y], self.query_cnt)));
                }
            }
        }
        self.segsz = 1;
        while self.segsz < self.query_cnt {
            self.segsz *= 2;
        }
        self.dfs(1, 0, self.segsz, (0..self.edges.len()).collect());
    }

    fn dfs(&mut self, k: usize, l: usize, r: usize, ids: Vec<usize>) {
        let mut st = vec![(k, l, r, ids)];
        while let Some((k, l, r, ids)) = st.pop() {
            if k == !0 {
                for _ in 0..l {
                    self.uf.undo();
                }
                continue;
            }
            let m = (l + r) / 2;
            let mut ls = vec![];
            let mut rs = vec![];
            let mut cnt = 0;
            for &i in &ids {
                let ((x, y), (a, b)) = self.edges[i];
                if b <= l || r <= a {
                    continue;
                }
                if a <= l && r <= b {
                    self.uf.add_edge(x, y);
                    cnt += 1;
                    continue;
                }
                if a < m {
                    ls.push(i);
                }
                if m < b {
                    rs.push(i);
                }
            }
            st.push((!0, cnt, !0, vec![]));
            if k < self.segsz {
                st.push((k * 2 + 1, m, r, rs));
                st.push((k * 2, l, m, ls));
            } else {
                while self.ri < self.reads.len() && self.reads[self.ri].0 == k - self.segsz {
                    self.uf.get(self.reads[self.ri].1.clone());
                    self.ri += 1;
                }
            }
        }
    }
}
