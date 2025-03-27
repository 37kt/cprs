use heavy_light_decomposition::HeavyLightDecomposition;
use static_top_tree::StaticTopTree;

pub trait DynamicTreeDpOperator {
    type Value;
    type Vertex;
    type Edge;

    fn unit() -> Self::Value;
    fn add_vertex(x: &Self::Value, v: &Self::Vertex) -> Self::Value;

    /// `x` は必ず単体の頂点であることが保証されている  
    /// (`rerooting_tree_dp` の `add_edge` とは異なる定義)
    fn add_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;

    /// `(a←b], (a←c] → (a←b]`
    fn rake(l: &Self::Value, r: &Self::Value) -> Self::Value;

    /// `(a←b], (b←c] → (a←c]`
    fn compress(p: &Self::Value, c: &Self::Value) -> Self::Value;
}

pub struct DynamicTreeDp<Op: DynamicTreeDpOperator> {
    hld: HeavyLightDecomposition,
    stt: static_top_tree::StaticTopTree,
    vertices: Vec<Op::Vertex>,
    edges: Vec<Op::Edge>,
    dp: Vec<Op::Value>,
}

impl<Op: DynamicTreeDpOperator> DynamicTreeDp<Op>
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

        let mut dp = vec![Op::unit(); n * 2 - 1];
        for v in 0..n {
            dp[v] = Op::add_vertex(&dp[v], &vertices[v]);
            if let Some(e) = hld.vertex_index(v).checked_sub(1) {
                dp[v] = Op::add_edge(&dp[v], &edges[e]);
            }
        }

        let mut tree = Self {
            hld,
            stt,
            vertices,
            edges,
            dp,
        };
        for v in n..n * 2 - 1 {
            tree.update(v);
        }
        tree
    }

    pub fn set_vertex(&mut self, v: usize, x: Op::Vertex) {
        self.vertices[v] = x;
        self.dp[v] = Op::add_vertex(&Op::unit(), &self.vertices[v]);
        if let Some(e) = self.hld.vertex_index(v).checked_sub(1) {
            self.dp[v] = Op::add_edge(&self.dp[v], &self.edges[e]);
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
        self.dp[v] = Op::add_edge(
            &Op::add_vertex(&Op::unit(), &self.vertices[v]),
            &self.edges[e],
        );
        let mut v = self.stt.nodes[v].par;
        while v != !0 {
            self.update(v);
            v = self.stt.nodes[v].par;
        }
    }

    pub fn fold(&self) -> Op::Value {
        self.dp.last().unwrap().clone()
    }

    fn update(&mut self, v: usize) {
        let l = &self.dp[self.stt.nodes[v].lch];
        let r = &self.dp[self.stt.nodes[v].rch];
        match self.stt.nodes[v].ty {
            static_top_tree::SttNodeType::Compress => {
                self.dp[v] = Op::compress(l, r);
            }
            static_top_tree::SttNodeType::Rake => {
                self.dp[v] = Op::rake(l, r);
            }
            _ => unreachable!(),
        }
    }
}

impl<Op: DynamicTreeDpOperator<Vertex = ()>> DynamicTreeDp<Op>
where
    Op::Value: Clone,
    Op::Edge: Clone,
{
    pub fn new(
        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge + Clone],
        root: usize,
    ) -> Self {
        let n = edges.len() + 1;
        DynamicTreeDp::with_vertices(edges, &vec![(); n], root)
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
    fn weight(&self) -> () {
        ()
    }
}
