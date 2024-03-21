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
    edge_cnt: BTreeMap<(usize, usize), usize>,
    appear: BTreeMap<(usize, usize), usize>,
    edges: Vec<((usize, usize), (usize, usize))>,
    reads: Vec<(usize, Q)>,
    ri: usize,
    segsz: usize,
    seg: Vec<Vec<(usize, usize)>>,
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
            edge_cnt: BTreeMap::new(),
            appear: BTreeMap::new(),
            edges: vec![],
            reads: vec![],
            ri: 0,
            segsz: 0,
            seg: vec![],
            uf,
        }
    }

    pub fn add_edge(&mut self, x: usize, y: usize) {
        let e = (x.min(y), x.max(y));
        self.edge_cnt.entry(e).and_modify(|e| *e += 1).or_insert(1);
        self.query_cnt += 1;
        if self.edge_cnt[&e] == 1 {
            self.appear.insert(e, self.query_cnt);
        }
    }

    pub fn remove_edge(&mut self, x: usize, y: usize) {
        let e = (x.min(y), x.max(y));
        self.edge_cnt.entry(e).and_modify(|e| *e -= 1);
        self.query_cnt += 1;
        if self.edge_cnt[&e] == 0 {
            self.edges.push((e, (self.appear[&e], self.query_cnt)));
        }
    }

    pub fn get(&mut self, query: Q) {
        self.reads.push((self.query_cnt, query));
    }

    pub fn run(&mut self) {
        for (&e, &c) in &self.edge_cnt {
            if c > 0 {
                self.edges.push((e, (self.appear[&e], self.query_cnt + 1)));
            }
        }
        self.segsz = 1;
        while self.segsz < self.query_cnt + 1 {
            self.segsz *= 2;
        }
        self.seg = vec![vec![]; self.segsz * 2];
        for &((x, y), (l, r)) in &self.edges.clone() {
            self.add(l, r, (x, y), 1, 0, self.segsz);
        }

        self.dfs(1);
    }

    fn add(&mut self, a: usize, b: usize, e: (usize, usize), k: usize, l: usize, r: usize) {
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.seg[k].push(e);
            return;
        }
        let m = (l + r) / 2;
        self.add(a, b, e, k * 2, l, m);
        self.add(a, b, e, k * 2 + 1, m, r);
    }

    fn dfs(&mut self, k: usize) {
        for &(x, y) in &self.seg[k] {
            self.uf.add_edge(x, y);
        }
        if k < self.segsz {
            self.dfs(k * 2);
            self.dfs(k * 2 + 1);
        } else {
            while self.ri < self.reads.len() && self.reads[self.ri].0 == k - self.segsz {
                self.uf.get(self.reads[self.ri].1.clone());
                self.ri += 1;
            }
        }
        for _ in 0..self.seg[k].len() {
            self.uf.undo();
        }
    }
}
