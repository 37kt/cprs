use csr_array::CsrArray;
use graph::Edge;

pub trait TreeDpOperator {
    type Value;
    type Vertex;
    type Edge;

    fn unit() -> Self::Value;
    fn add_vertex(x: &Self::Value, v: &Self::Vertex) -> Self::Value;
    fn add_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;
    fn rake(x: &Self::Value, y: &Self::Value) -> Self::Value;
}

pub struct RerootingTreeDp<Op: TreeDpOperator> {
    par: Vec<usize>,
    dp: Vec<Op::Value>,
    dpc: Vec<Op::Value>,
    dpp: Vec<Op::Value>,
}

impl<Op: TreeDpOperator<Vertex = ()>> RerootingTreeDp<Op> {
    pub fn new(g: &CsrArray<impl Edge<Op::Edge>>) -> Self {
        Self::with_vertices(g, &vec![(); g.len()])
    }
}

impl<Op: TreeDpOperator> RerootingTreeDp<Op> {
    pub fn with_vertices(g: &CsrArray<impl Edge<Op::Edge>>, vs: &[Op::Vertex]) -> Self {
        let n = g.len();
        if n == 0 {
            return Self {
                par: vec![],
                dp: vec![],
                dpc: vec![],
                dpp: vec![],
            };
        }
        assert_eq!(n, vs.len());
        assert_eq!((n - 1) * 2, g.flat_len(), "g must be a undirected tree");

        let mut par = vec![!0; n];
        let mut ord = Vec::with_capacity(n); // BFS 順
        ord.push(0);
        for i in 0..n {
            let v = ord[i];
            for e in &g[v] {
                let u = e.to();
                if u == par[v] {
                    continue;
                }
                par[u] = v;
                ord.push(u);
            }
        }

        let mut dpc = (0..n).map(|_| Op::unit()).collect::<Vec<_>>();
        let mut dpp = (0..n).map(|_| Op::unit()).collect::<Vec<_>>();
        let mut dp = (0..n).map(|_| Op::unit()).collect::<Vec<_>>();
        for &v in ord.iter().rev() {
            let mut s = Op::unit();
            for e in &g[v] {
                let u = e.to();
                let w = e.weight();
                if u == par[v] {
                    continue;
                }
                let x = Op::add_vertex(&dpc[u], &vs[u]);
                dp[u] = Op::add_edge(&x, w);
                dpp[u] = s;
                s = Op::rake(&dpp[u], &dp[u]);
            }
            std::mem::swap(&mut dpc[v], &mut s);
            for e in g[v].iter().rev() {
                let u = e.to();
                if u == par[v] {
                    continue;
                }
                dpp[u] = Op::rake(&dpp[u], &s);
                s = Op::rake(&dp[u], &s);
            }
        }

        // この時点では
        // dpc[v] = 部分木 v の v 抜き
        // dpp[v] = 部分木 p の p, v 抜き

        for &v in &ord {
            for e in &g[v] {
                let u = e.to();
                let w = e.weight();
                if u == par[v] {
                    continue;
                }
                dpp[u] = Op::add_vertex(&Op::rake(&dpp[u], &dp[v]), &vs[v]);
                dp[u] = Op::add_edge(&dpp[u], w);
            }
            dp[v] = Op::add_vertex(&Op::rake(&dp[v], &dpc[v]), &vs[v]);
            dpc[v] = Op::add_vertex(&dpc[v], &vs[v]);
        }

        Self { par, dp, dpc, dpp }
    }

    pub fn fold_all(&self, v: usize) -> &Op::Value {
        &self.dp[v]
    }

    pub fn fold_subtree(&self, v: usize, p: usize) -> &Op::Value {
        let n = self.par.len();
        assert!(v < n && p < n);
        if self.par[v] == p {
            &self.dpc[v]
        } else if self.par[p] == v {
            &self.dpp[v]
        } else {
            panic!("v and p must be directly connected by an edge.")
        }
    }
}

impl<'a, Op: TreeDpOperator> IntoIterator for &'a RerootingTreeDp<Op> {
    type Item = &'a Op::Value;
    type IntoIter = std::slice::Iter<'a, Op::Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.dp.iter()
    }
}
