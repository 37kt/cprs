use graph::UndirectedGraph;
use static_top_tree::StaticTopTree;

pub trait DynamicRerootingTreeDPOperator {
    type V: Clone;
    type E: Clone;
    type X: Clone;

    fn e() -> Self::X;

    /// 頂点とその親への辺からなるクラスター ( (o←a\], (o→a\] )  
    /// o は virtual
    fn single(v: &Self::V, e: Option<&Self::E>) -> (Self::X, Self::X);

    /// (a←b\], (a←c\] → (a←b\]
    fn rake(l: &Self::X, r: &Self::X) -> Self::X;

    /// (a→b\], (a←c\] → (a→b\]
    fn rake2(l: &Self::X, r: &Self::X) -> Self::X;

    /// (a→b\], (b←c\] → (a→b\]
    fn rake3(p: &Self::X, c: &Self::X) -> Self::X;

    /// (a←b\], (b←c\] → (a←c\]
    fn compress(p: &Self::X, c: &Self::X) -> Self::X;

    /// (a←b\], (b→c\] → (a→c\]
    fn compress2(p: &Self::X, c: &Self::X) -> Self::X;
}

pub struct DynamicRerootingTreeDP<Op>
where
    Op: DynamicRerootingTreeDPOperator,
    Op::X: Clone,
{
    stt: StaticTopTree,
    vertices: Vec<Op::V>,
    edges: Vec<Op::E>,
    dp: Vec<(Op::X, Op::X)>,
}

impl<Op> DynamicRerootingTreeDP<Op>
where
    Op: DynamicRerootingTreeDPOperator,
    Op::X: Clone,
{
    pub fn new(g: &UndirectedGraph<Op::V, Op::E>, root: usize) -> Self
    where
        Op::V: Clone,
        Op::E: Clone,
    {
        let n = g.len();
        let stt = StaticTopTree::new(g, root);
        let vertices = (0..n).map(|i| g.vertex(i).clone()).collect::<Vec<_>>();
        let mut ord = vec![(0, 0); n - 1];
        for u in 0..n {
            for (i, (v, _)) in g[u].iter().enumerate() {
                if u < *v {
                    let j = stt.hld.edge_index(u, *v);
                    ord[j] = (u, i);
                }
            }
        }
        let edges = ord
            .into_iter()
            .map(|(v, i)| g[v][i].1.clone())
            .collect::<Vec<_>>();

        let mut dp = (0..n * 2 - 1)
            .map(|_| (Op::e(), Op::e()))
            .collect::<Vec<_>>();
        for v in 0..n {
            // vertex_index(v) - 1 が親への辺の index
            let i = stt.hld.vertex_index(v);
            if i == 0 {
                dp[v] = Op::single(&vertices[v], None);
            } else {
                dp[v] = Op::single(&vertices[v], Some(&edges[i - 1]));
            }
        }

        let mut treedp = Self {
            stt,
            vertices,
            edges,
            dp,
        };

        for v in n..n * 2 - 1 {
            treedp.update(v);
        }

        treedp
    }

    pub fn set_vertex(&mut self, v: usize, x: Op::V) {
        self.vertices[v] = x;
        let i = self.stt.hld.vertex_index(v);
        if i == 0 {
            self.dp[v] = Op::single(&self.vertices[v], None);
        } else {
            self.dp[v] = Op::single(&self.vertices[v], Some(&self.edges[i - 1]));
        }
        let mut v = self.stt.par[v];
        while v != !0 {
            self.update(v);
            v = self.stt.par[v];
        }
    }

    pub fn set_edge(&mut self, u: usize, v: usize, x: Op::E) {
        let i = self.stt.hld.edge_index(u, v);
        self.edges[i] = x;
        let v = if self.stt.hld.parent(u) == v { u } else { v };
        self.dp[v] = Op::single(&self.vertices[v], Some(&self.edges[i]));
        let mut v = self.stt.par[v];
        while v != !0 {
            self.update(v);
            v = self.stt.par[v];
        }
    }

    pub fn prod(&self, mut v: usize) -> Op::X {
        let mut a = self.dp[v].1.clone(); // 上から compress を受け入れるときに使う
        let mut b = Op::e(); // 下から compress を受け入れるときに使う
        let mut c = Op::e(); // 左から rake されるときの退避先 (?)
        loop {
            let p = self.stt.par[v];
            if p == !0 {
                break;
            }
            let l = self.stt.lch[p];
            let r = self.stt.rch[p];
            if self.stt.is_compress[p] {
                if l == v {
                    b = Op::compress(&b, &self.dp[r].0);
                } else {
                    a = Op::compress2(&self.dp[l].1, &a);
                }
            } else {
                if l == v {
                    a = Op::rake2(&a, &self.dp[r].0);
                } else {
                    c = Op::compress2(&Op::rake3(&a, &b), &c);
                    a = Op::e();
                    b = self.dp[l].0.clone();
                }
            }
            v = p;
        }
        Op::compress2(&Op::rake3(&a, &b), &c)
    }

    fn update(&mut self, v: usize) {
        let (l1, l2) = &self.dp[self.stt.lch[v]];
        let (r1, r2) = &self.dp[self.stt.rch[v]];
        if self.stt.is_compress[v] {
            self.dp[v] = (Op::compress(l1, r1), Op::compress2(l2, r2));
        } else {
            self.dp[v] = (Op::rake(l1, r1), Op::rake2(l2, r1));
        }
    }
}
