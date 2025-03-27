use csr_array::CsrArray;
use graph::Edge;

#[derive(Debug)]
pub struct CentroidDecomposition {
    pub root: usize,
    pub vs: [Vec<usize>; 2],
    pub par: [Vec<usize>; 2],
}

impl CentroidDecomposition {
    pub fn solve<W>(g: &CsrArray<impl Edge<W>>, mut f: impl FnMut(&CentroidDecomposition)) {
        let mut cd = CdImpl::new(g);
        cd.dfs_decompose(0, &mut f);
    }

    fn new() -> Self {
        Self {
            root: !0,
            vs: [vec![], vec![]],
            par: [vec![], vec![]],
        }
    }

    fn clear(&mut self) {
        self.root = !0;
        for i in 0..2 {
            self.vs[i].clear();
            self.par[i].clear();
        }
    }
}

struct CdImpl {
    edges: Vec<usize>,
    head: Vec<usize>,
    tail: Vec<usize>,

    sz: Vec<usize>,
    cd: CentroidDecomposition,
}

impl CdImpl {
    fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {
        let n = g.len();
        let mut head = vec![0; n];
        let mut tail = vec![0; n];
        for i in 0..n {
            tail[i] = g[i].len();
            if i > 0 {
                tail[i] += tail[i - 1];
                head[i] = tail[i - 1];
            }
        }
        let edges = g.iter().flatten().map(|e| e.to()).collect();
        Self {
            edges,
            head,
            tail,
            sz: vec![0; n],
            cd: CentroidDecomposition::new(),
        }
    }

    fn dfs_sz(&mut self, v: usize, p: usize) {
        self.sz[v] = 1;
        for i in self.head[v]..self.tail[v] {
            let u = self.edges[i];
            if u != p {
                self.dfs_sz(u, v);
                self.sz[v] += self.sz[u];
            }
        }
    }

    fn dfs_centroid(&mut self, v: usize, p: usize, mid: usize) -> usize {
        for i in self.head[v]..self.tail[v] {
            let u = self.edges[i];
            if u != p && self.sz[u] > mid {
                return self.dfs_centroid(u, v, mid);
            }
        }
        v
    }

    fn dfs_build_tree(&mut self, color: usize, v: usize, p: usize) {
        self.cd.vs[color].push(v);
        self.cd.par[color].push(p);
        for i in self.head[v]..self.tail[v] {
            let u = self.edges[i];
            if u != p {
                self.dfs_build_tree(color, u, v);
            }
        }
    }

    fn dfs_decompose(&mut self, v: usize, f: &mut impl FnMut(&CentroidDecomposition)) {
        self.dfs_sz(v, !0);
        if self.sz[v] <= 2 {
            return;
        }
        let c = self.dfs_centroid(v, !0, self.sz[v] / 2);
        self.dfs_sz(c, !0);

        let mut sum_sz = 0;
        let mut m = self.head[c];
        for i in self.head[c]..self.tail[c] {
            let v = self.edges[i];
            if sum_sz + self.sz[v] <= (self.sz[c] - 1) / 2 {
                sum_sz += self.sz[v];
                self.edges.swap(m, i);
                m += 1;
            }
        }

        self.cd.clear();
        self.cd.root = c;
        for i in self.head[c]..self.tail[c] {
            let v = self.edges[i];
            let color = if i < m { 0 } else { 1 };
            self.dfs_build_tree(color, v, c);
        }
        f(&self.cd);

        std::mem::swap(&mut self.head[c], &mut m);
        self.dfs_decompose(c, f);
        std::mem::swap(&mut self.head[c], &mut m);
        std::mem::swap(&mut self.tail[c], &mut m);
        self.dfs_decompose(c, f);
        std::mem::swap(&mut self.tail[c], &mut m);
    }
}
