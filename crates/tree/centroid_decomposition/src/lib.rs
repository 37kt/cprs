use csr_array::CsrArray;
use graph::Edge;

#[derive(Debug, Clone, Copy)]
pub struct CentroidDecomposition<'a> {
    pub root: usize,
    pub vs: &'a [usize],
    pub mid: usize,
    pub par: &'a [usize],
}

impl<'a> CentroidDecomposition<'a> {
    pub fn solve<W>(g: &CsrArray<impl Edge<W>>, mut f: impl FnMut(CentroidDecomposition)) {
        let mut cd = CdImpl::new(g);
        cd.dfs_decompose(0, &mut f);
    }
}

struct CdImpl {
    edges: Vec<usize>,
    head: Vec<usize>,
    tail: Vec<usize>,

    sz: Vec<usize>,
    par: Vec<usize>,
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
            par: vec![!0; n],
        }
    }

    fn dfs_decompose(&mut self, v: usize, f: &mut impl FnMut(CentroidDecomposition)) {
        self.par[v] = !0;
        let mut ord = vec![v];
        let mut i = 0;
        while i < ord.len() {
            let v = ord[i];
            self.sz[v] = 1;
            for &u in &self.edges[self.head[v]..self.tail[v]] {
                if u != self.par[v] {
                    self.par[u] = v;
                    ord.push(u);
                }
            }
            i += 1;
        }

        let n = ord.len();
        if n <= 2 {
            return;
        }

        let mut c = !0;
        for &v in ord.iter().rev() {
            if self.par[v] != !0 {
                self.sz[self.par[v]] += self.sz[v];
            }
            if c == !0 && self.sz[v] >= (n + 1) / 2 {
                c = v;
            }
        }

        let mut p = !0;
        let mut p_sz = 0;
        let mut v = c;
        while v != !0 {
            std::mem::swap(&mut self.sz[v], &mut p_sz);
            self.sz[v] = n - self.sz[v];
            std::mem::swap(&mut self.par[v], &mut p);
            std::mem::swap(&mut p, &mut v);
        }

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

        let mut vs = Vec::with_capacity(n - 1);
        let mut mid = 0;
        for (color, cs) in [
            (0, &self.edges[self.head[c]..m]),
            (1, &self.edges[m..self.tail[c]]),
        ] {
            for &v in cs {
                vs.push(v);
            }
            let mut i = mid;
            while i < vs.len() {
                let v = vs[i];
                for &u in &self.edges[self.head[v]..self.tail[v]] {
                    if u != self.par[v] {
                        vs.push(u);
                    }
                }
                i += 1;
            }
            if color == 0 {
                mid = vs.len();
            }
        }
        f(CentroidDecomposition {
            root: c,
            vs: &vs,
            mid,
            par: &self.par,
        });

        std::mem::swap(&mut self.head[c], &mut m);
        self.dfs_decompose(c, f);
        std::mem::swap(&mut self.head[c], &mut m);
        std::mem::swap(&mut self.tail[c], &mut m);
        self.dfs_decompose(c, f);
        std::mem::swap(&mut self.tail[c], &mut m);
    }
}
