use std::collections::BTreeMap;

use rollback_union_find::RollbackUnionFind;

enum ReadQuery {
    Leader(usize),
    Same(usize, usize),
    Size(usize),
    Count,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryResult {
    Leader(usize),
    Same(bool),
    Size(usize),
    Count(usize),
}

pub struct OfflineDynamicConnectivity {
    built: bool,
    query_cnt: usize,
    component_cnt: usize,
    edge_cnt: BTreeMap<(usize, usize), usize>,
    appear: BTreeMap<(usize, usize), usize>,
    edges: Vec<((usize, usize), (usize, usize))>,
    reads: Vec<(usize, ReadQuery)>,
    ri: usize,
    res: Vec<QueryResult>,

    segsz: usize,
    seg: Vec<Vec<(usize, usize)>>,

    uf: RollbackUnionFind,
}

impl OfflineDynamicConnectivity {
    pub fn new(n: usize) -> Self {
        Self {
            built: false,
            query_cnt: 0,
            component_cnt: n,
            edge_cnt: BTreeMap::new(),
            appear: BTreeMap::new(),
            edges: vec![],
            reads: vec![],
            ri: 0,
            res: vec![],

            segsz: 0,
            seg: vec![],

            uf: RollbackUnionFind::new(n),
        }
    }

    pub fn add_edge(&mut self, x: usize, y: usize) {
        assert!(!self.built);
        let e = (x.min(y), x.max(y));
        self.edge_cnt.entry(e).and_modify(|e| *e += 1).or_insert(1);
        if self.edge_cnt[&e] == 1 {
            self.appear.insert(e, self.query_cnt);
        }
        self.query_cnt += 1;
    }

    pub fn remove_edge(&mut self, x: usize, y: usize) {
        assert!(!self.built);
        let e = (x.min(y), x.max(y));
        self.edge_cnt.entry(e).and_modify(|e| *e -= 1);
        if self.edge_cnt[&e] == 0 {
            self.edges.push((e, (self.appear[&e], self.query_cnt)));
        }
        self.query_cnt += 1;
    }

    pub fn leader(&mut self, x: usize) -> usize {
        assert!(!self.built);
        self.reads.push((self.query_cnt, ReadQuery::Leader(x)));
        self.query_cnt += 1;
        self.reads.len() - 1
    }

    pub fn same(&mut self, x: usize, y: usize) -> usize {
        assert!(!self.built);
        self.reads.push((self.query_cnt, ReadQuery::Same(x, y)));
        self.query_cnt += 1;
        self.reads.len() - 1
    }

    pub fn size(&mut self, x: usize) -> usize {
        assert!(!self.built);
        self.reads.push((self.query_cnt, ReadQuery::Size(x)));
        self.query_cnt += 1;
        self.reads.len() - 1
    }

    pub fn count(&mut self) -> usize {
        assert!(!self.built);
        self.reads.push((self.query_cnt, ReadQuery::Count));
        self.query_cnt += 1;
        self.reads.len() - 1
    }

    pub fn build(&mut self) -> Vec<QueryResult> {
        assert!(!self.built);
        self.built = true;

        for (&e, &c) in &self.edge_cnt {
            if c > 0 {
                self.edges.push((e, (self.appear[&e], self.query_cnt)));
            }
        }
        self.segsz = 1;
        while self.segsz < self.query_cnt {
            self.segsz *= 2;
        }
        self.seg = vec![vec![]; self.segsz * 2];
        for &((x, y), (l, r)) in &self.edges.clone() {
            self.add(l, r, (x, y), 1, 0, self.segsz);
        }

        self.dfs(1);
        self.res.clone()
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
        let mut merge_cnt = 0;
        for &(x, y) in &self.seg[k] {
            if self.uf.merge(x, y) {
                merge_cnt += 1;
            }
        }
        self.component_cnt -= merge_cnt;
        if k < self.segsz {
            self.dfs(k * 2);
            self.dfs(k * 2 + 1);
        } else if self.ri < self.reads.len() && self.reads[self.ri].0 == k - self.segsz {
            match self.reads[self.ri].1 {
                ReadQuery::Leader(x) => {
                    self.res.push(QueryResult::Leader(self.uf.leader(x)));
                }
                ReadQuery::Same(x, y) => {
                    self.res.push(QueryResult::Same(self.uf.same(x, y)));
                }
                ReadQuery::Size(x) => {
                    self.res.push(QueryResult::Size(self.uf.size(x)));
                }
                ReadQuery::Count => {
                    self.res.push(QueryResult::Count(self.component_cnt));
                }
            }
            self.ri += 1;
        }
        for _ in 0..self.seg[k].len() {
            self.uf.undo();
        }
        self.component_cnt += merge_cnt;
    }
}
