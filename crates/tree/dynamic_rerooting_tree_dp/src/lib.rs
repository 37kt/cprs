// https://maspypy.com/library-checker-point-set-tree-path-composite-sum

use heavy_light_decomposition::HeavyLightDecomposition;
use static_top_tree::StaticTopTree;

pub trait DynamicRerootingTreeDpOperator {
    type Value;
    type Vertex;
    type Edge;

    fn unit() -> Self::Value;

    /// 値 `v` の頂点 1 つからなるクラスターを生成する<br>
    /// `v → [a]`
    fn vertex(v: &Self::Vertex) -> Self::Value;

    /// 頂点 1 つからなるクラスターの上に上向きの辺を追加する<br>
    /// `x` は頂点 1 つからなるクラスターであることが保証されている<br>
    /// `[a] → (o←a]`
    fn add_up_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;

    /// 頂点 1 つからなるクラスターの上に下向きの辺を追加する<br>
    /// `x` は頂点 1 つからなるクラスターであることが保証されている<br>
    /// `[a] → (o→a]`
    fn add_down_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;

    /// `(a←b\], (a←c\] → (a←b\]`
    fn rake1(l: &Self::Value, r: &Self::Value) -> Self::Value;

    /// `(a→b\], (a←c\] → (a→b\]`
    fn rake2(l: &Self::Value, r: &Self::Value) -> Self::Value;

    /// `(a→b\], (b←c\] → (a→b\]`
    fn rake3(p: &Self::Value, c: &Self::Value) -> Self::Value;

    /// `(a←b\], (b←c\] → (a←c\]`
    fn compress1(p: &Self::Value, c: &Self::Value) -> Self::Value;

    /// `(a←b\], (b→c\] → (a→c\]`
    fn compress2(p: &Self::Value, c: &Self::Value) -> Self::Value;
}

pub struct DynamicRerootingTreeDp<Op: DynamicRerootingTreeDpOperator> {
    hld: HeavyLightDecomposition,
    stt: static_top_tree::StaticTopTree,
    vertices: Vec<Op::Vertex>,
    edges: Vec<Op::Edge>,
    dp_up: Vec<Op::Value>,
    dp_down: Vec<Op::Value>,
}

impl<Op: DynamicRerootingTreeDpOperator> DynamicRerootingTreeDp<Op>
where
    Op::Value: Clone,
    Op::Vertex: Clone,
    Op::Edge: Clone,
{
    pub fn with_vertices(
        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge + Clone],
        vertices: &[Op::Vertex],
        root: usize,
    ) -> Self {
        let n = vertices.len();
        assert_eq!(n, edges.len() + 1);
        let hld = HeavyLightDecomposition::from_edges(edges, root);
        let edges = hld
            .edges_order()
            .map(|e| edges[e].weight())
            .collect::<Vec<_>>();
        let vertices = vertices.to_vec();
        let stt = StaticTopTree::new(&hld);

        let mut dp_up = vec![Op::unit(); n * 2 - 1];
        let mut dp_down = vec![Op::unit(); n * 2 - 1];
        for v in 0..n {
            dp_up[v] = Op::vertex(&vertices[v]);
            dp_down[v] = Op::vertex(&vertices[v]);
            if let Some(e) = hld.vertex_index(v).checked_sub(1) {
                dp_up[v] = Op::add_up_edge(&dp_up[v], &edges[e]);
                dp_down[v] = Op::add_down_edge(&dp_down[v], &edges[e]);
            }
        }

        let mut tree = Self {
            hld,
            stt,
            vertices,
            edges,
            dp_up,
            dp_down,
        };
        for v in n..n * 2 - 1 {
            tree.update(v);
        }
        tree
    }

    pub fn set_vertex(&mut self, v: usize, x: Op::Vertex) {
        self.vertices[v] = x;
        self.dp_up[v] = Op::vertex(&self.vertices[v]);
        self.dp_down[v] = Op::vertex(&self.vertices[v]);
        if let Some(e) = self.hld.vertex_index(v).checked_sub(1) {
            self.dp_up[v] = Op::add_up_edge(&self.dp_up[v], &self.edges[e]);
            self.dp_down[v] = Op::add_down_edge(&self.dp_down[v], &self.edges[e]);
        }
        let mut v = self.stt.nodes[v].par;
        while v != !0 {
            self.update(v);
            v = self.stt.nodes[v].par;
        }
    }

    pub fn set_edge(&mut self, u: usize, v: usize, x: Op::Edge) {
        let e = self.hld.edge_index(u, v);
        self.edges[e] = x;
        let v = if self.hld.parent(u) == Some(v) { u } else { v };
        self.dp_up[v] = Op::add_up_edge(&Op::vertex(&self.vertices[v]), &self.edges[e]);
        self.dp_down[v] = Op::add_down_edge(&Op::vertex(&self.vertices[v]), &self.edges[e]);
        let mut v = self.stt.nodes[v].par;
        while v != !0 {
            self.update(v);
            v = self.stt.nodes[v].par;
        }
    }

    pub fn fold(&self, mut v: usize) -> Op::Value {
        let mut a = self.dp_down[v].clone(); // 上から compress を受け入れるときに使う
        let mut b = Op::unit(); // 下から compress を受け入れるときに使う
        let mut c = Op::unit(); // 左から rake されるときの退避先

        loop {
            let p = self.stt.nodes[v].par;
            if p == !0 {
                break;
            }
            let l = self.stt.nodes[p].lch;
            let r = self.stt.nodes[p].rch;

            match self.stt.nodes[p].ty {
                static_top_tree::SttNodeType::Compress => {
                    if l == v {
                        b = Op::compress1(&b, &self.dp_up[r]);
                    } else {
                        a = Op::compress2(&self.dp_down[l], &a);
                    }
                }
                static_top_tree::SttNodeType::Rake => {
                    if l == v {
                        a = Op::rake2(&a, &self.dp_up[r]);
                    } else {
                        c = Op::compress2(&Op::rake3(&a, &b), &c);
                        a = Op::unit();
                        b = self.dp_up[l].clone();
                    }
                }
                _ => unreachable!(),
            }
            v = p;
        }
        Op::compress2(&Op::rake3(&a, &b), &c)
    }

    fn update(&mut self, v: usize) {
        let lu = &self.dp_up[self.stt.nodes[v].lch];
        let ld = &self.dp_down[self.stt.nodes[v].lch];
        let ru = &self.dp_up[self.stt.nodes[v].rch];
        let rd = &self.dp_down[self.stt.nodes[v].rch];
        match self.stt.nodes[v].ty {
            static_top_tree::SttNodeType::Compress => {
                (self.dp_up[v], self.dp_down[v]) = (Op::compress1(lu, ru), Op::compress2(ld, rd));
            }
            static_top_tree::SttNodeType::Rake => {
                (self.dp_up[v], self.dp_down[v]) = (Op::rake1(lu, ru), Op::rake2(ld, ru));
            }
            _ => unreachable!(),
        }
    }
}

impl<Op: DynamicRerootingTreeDpOperator<Vertex = ()>> DynamicRerootingTreeDp<Op>
where
    Op::Value: Clone,
    Op::Edge: Clone,
{
    pub fn new(
        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge + Clone],
        root: usize,
    ) -> Self {
        let n = edges.len() + 1;
        DynamicRerootingTreeDp::with_vertices(edges, &vec![(); n], root)
    }
}

#[doc(hidden)]
pub trait Edge<Weight> {
    fn weight(&self) -> Weight;
}

#[doc(hidden)]
impl<Weight: Clone> Edge<Weight> for (usize, usize, Weight) {
    fn weight(&self) -> Weight {
        self.2.clone()
    }
}

#[doc(hidden)]
impl Edge<()> for (usize, usize) {
    fn weight(&self) {}
}
