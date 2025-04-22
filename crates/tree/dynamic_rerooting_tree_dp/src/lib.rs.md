---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/builder.rs
    title: crates/data_structure/csr_array/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/csr_array.rs
    title: crates/data_structure/csr_array/src/csr_array.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/csr_array/src/lib.rs
    title: crates/data_structure/csr_array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  - icon: ':warning:'
    path: crates/tree/static_top_tree/src/lib.rs
    title: crates/tree/static_top_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/point_set_tree_path_composite_sum/src/main.rs
    title: verify/library_checker/tree/point_set_tree_path_composite_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://maspypy.com/library-checker-point-set-tree-path-composite-sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://maspypy.com/library-checker-point-set-tree-path-composite-sum\n\
    \nuse heavy_light_decomposition::HeavyLightDecomposition;\nuse static_top_tree::StaticTopTree;\n\
    \npub trait DynamicRerootingTreeDpOperator {\n    type Value;\n    type Vertex;\n\
    \    type Edge;\n\n    fn unit() -> Self::Value;\n\n    /// \u5024 `v` \u306E\u9802\
    \u70B9 1 \u3064\u304B\u3089\u306A\u308B\u30AF\u30E9\u30B9\u30BF\u30FC\u3092\u751F\
    \u6210\u3059\u308B<br>\n    /// `v \u2192 [a]`\n    fn vertex(v: &Self::Vertex)\
    \ -> Self::Value;\n\n    /// \u9802\u70B9 1 \u3064\u304B\u3089\u306A\u308B\u30AF\
    \u30E9\u30B9\u30BF\u30FC\u306E\u4E0A\u306B\u4E0A\u5411\u304D\u306E\u8FBA\u3092\
    \u8FFD\u52A0\u3059\u308B<br>\n    /// `x` \u306F\u9802\u70B9 1 \u3064\u304B\u3089\
    \u306A\u308B\u30AF\u30E9\u30B9\u30BF\u30FC\u3067\u3042\u308B\u3053\u3068\u304C\
    \u4FDD\u8A3C\u3055\u308C\u3066\u3044\u308B<br>\n    /// `[a] \u2192 (o\u2190a]`\n\
    \    fn add_up_edge(x: &Self::Value, e: &Self::Edge) -> Self::Value;\n\n    ///\
    \ \u9802\u70B9 1 \u3064\u304B\u3089\u306A\u308B\u30AF\u30E9\u30B9\u30BF\u30FC\u306E\
    \u4E0A\u306B\u4E0B\u5411\u304D\u306E\u8FBA\u3092\u8FFD\u52A0\u3059\u308B<br>\n\
    \    /// `x` \u306F\u9802\u70B9 1 \u3064\u304B\u3089\u306A\u308B\u30AF\u30E9\u30B9\
    \u30BF\u30FC\u3067\u3042\u308B\u3053\u3068\u304C\u4FDD\u8A3C\u3055\u308C\u3066\
    \u3044\u308B<br>\n    /// `[a] \u2192 (o\u2192a]`\n    fn add_down_edge(x: &Self::Value,\
    \ e: &Self::Edge) -> Self::Value;\n\n    /// `(a\u2190b\\], (a\u2190c\\] \u2192\
    \ (a\u2190b\\]`\n    fn rake1(l: &Self::Value, r: &Self::Value) -> Self::Value;\n\
    \n    /// `(a\u2192b\\], (a\u2190c\\] \u2192 (a\u2192b\\]`\n    fn rake2(l: &Self::Value,\
    \ r: &Self::Value) -> Self::Value;\n\n    /// `(a\u2192b\\], (b\u2190c\\] \u2192\
    \ (a\u2192b\\]`\n    fn rake3(p: &Self::Value, c: &Self::Value) -> Self::Value;\n\
    \n    /// `(a\u2190b\\], (b\u2190c\\] \u2192 (a\u2190c\\]`\n    fn compress1(p:\
    \ &Self::Value, c: &Self::Value) -> Self::Value;\n\n    /// `(a\u2190b\\], (b\u2192\
    c\\] \u2192 (a\u2192c\\]`\n    fn compress2(p: &Self::Value, c: &Self::Value)\
    \ -> Self::Value;\n}\n\npub struct DynamicRerootingTreeDp<Op: DynamicRerootingTreeDpOperator>\
    \ {\n    hld: HeavyLightDecomposition,\n    stt: static_top_tree::StaticTopTree,\n\
    \    vertices: Vec<Op::Vertex>,\n    edges: Vec<Op::Edge>,\n    dp_up: Vec<Op::Value>,\n\
    \    dp_down: Vec<Op::Value>,\n}\n\nimpl<Op: DynamicRerootingTreeDpOperator> DynamicRerootingTreeDp<Op>\n\
    where\n    Op::Value: Clone,\n    Op::Vertex: Clone,\n    Op::Edge: Clone,\n{\n\
    \    pub fn with_vertices(\n        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge\
    \ + Clone],\n        vertices: &[Op::Vertex],\n        root: usize,\n    ) ->\
    \ Self {\n        let n = vertices.len();\n        assert_eq!(n, edges.len() +\
    \ 1);\n        let hld = HeavyLightDecomposition::from_edges(edges, root);\n \
    \       let edges = hld\n            .edges_order()\n            .map(|e| edges[e].weight())\n\
    \            .collect::<Vec<_>>();\n        let vertices = vertices.to_vec();\n\
    \        let stt = StaticTopTree::new(&hld);\n\n        let mut dp_up = vec![Op::unit();\
    \ n * 2 - 1];\n        let mut dp_down = vec![Op::unit(); n * 2 - 1];\n      \
    \  for v in 0..n {\n            dp_up[v] = Op::vertex(&vertices[v]);\n       \
    \     dp_down[v] = Op::vertex(&vertices[v]);\n            if let Some(e) = hld.vertex_index(v).checked_sub(1)\
    \ {\n                dp_up[v] = Op::add_up_edge(&dp_up[v], &edges[e]);\n     \
    \           dp_down[v] = Op::add_down_edge(&dp_down[v], &edges[e]);\n        \
    \    }\n        }\n\n        let mut tree = Self {\n            hld,\n       \
    \     stt,\n            vertices,\n            edges,\n            dp_up,\n  \
    \          dp_down,\n        };\n        for v in n..n * 2 - 1 {\n           \
    \ tree.update(v);\n        }\n        tree\n    }\n\n    pub fn set_vertex(&mut\
    \ self, v: usize, x: Op::Vertex) {\n        self.vertices[v] = x;\n        self.dp_up[v]\
    \ = Op::vertex(&self.vertices[v]);\n        self.dp_down[v] = Op::vertex(&self.vertices[v]);\n\
    \        if let Some(e) = self.hld.vertex_index(v).checked_sub(1) {\n        \
    \    self.dp_up[v] = Op::add_up_edge(&self.dp_up[v], &self.edges[e]);\n      \
    \      self.dp_down[v] = Op::add_down_edge(&self.dp_down[v], &self.edges[e]);\n\
    \        }\n        let mut v = self.stt.nodes[v].par;\n        while v != !0\
    \ {\n            self.update(v);\n            v = self.stt.nodes[v].par;\n   \
    \     }\n    }\n\n    pub fn set_edge(&mut self, u: usize, v: usize, x: Op::Edge)\
    \ {\n        let e = self.hld.edge_index(u, v);\n        self.edges[e] = x;\n\
    \        let v = if self.hld.parent(u) == Some(v) { u } else { v };\n        self.dp_up[v]\
    \ = Op::add_up_edge(&Op::vertex(&self.vertices[v]), &self.edges[e]);\n       \
    \ self.dp_down[v] = Op::add_down_edge(&Op::vertex(&self.vertices[v]), &self.edges[e]);\n\
    \        let mut v = self.stt.nodes[v].par;\n        while v != !0 {\n       \
    \     self.update(v);\n            v = self.stt.nodes[v].par;\n        }\n   \
    \ }\n\n    pub fn fold(&self, mut v: usize) -> Op::Value {\n        let mut a\
    \ = self.dp_down[v].clone(); // \u4E0A\u304B\u3089 compress \u3092\u53D7\u3051\
    \u5165\u308C\u308B\u3068\u304D\u306B\u4F7F\u3046\n        let mut b = Op::unit();\
    \ // \u4E0B\u304B\u3089 compress \u3092\u53D7\u3051\u5165\u308C\u308B\u3068\u304D\
    \u306B\u4F7F\u3046\n        let mut c = Op::unit(); // \u5DE6\u304B\u3089 rake\
    \ \u3055\u308C\u308B\u3068\u304D\u306E\u9000\u907F\u5148\n\n        loop {\n \
    \           let p = self.stt.nodes[v].par;\n            if p == !0 {\n       \
    \         break;\n            }\n            let l = self.stt.nodes[p].lch;\n\
    \            let r = self.stt.nodes[p].rch;\n\n            match self.stt.nodes[p].ty\
    \ {\n                static_top_tree::SttNodeType::Compress => {\n           \
    \         if l == v {\n                        b = Op::compress1(&b, &self.dp_up[r]);\n\
    \                    } else {\n                        a = Op::compress2(&self.dp_down[l],\
    \ &a);\n                    }\n                }\n                static_top_tree::SttNodeType::Rake\
    \ => {\n                    if l == v {\n                        a = Op::rake2(&a,\
    \ &self.dp_up[r]);\n                    } else {\n                        c =\
    \ Op::compress2(&Op::rake3(&a, &b), &c);\n                        a = Op::unit();\n\
    \                        b = self.dp_up[l].clone();\n                    }\n \
    \               }\n                _ => unreachable!(),\n            }\n     \
    \       v = p;\n        }\n        Op::compress2(&Op::rake3(&a, &b), &c)\n   \
    \ }\n\n    fn update(&mut self, v: usize) {\n        let lu = &self.dp_up[self.stt.nodes[v].lch];\n\
    \        let ld = &self.dp_down[self.stt.nodes[v].lch];\n        let ru = &self.dp_up[self.stt.nodes[v].rch];\n\
    \        let rd = &self.dp_down[self.stt.nodes[v].rch];\n        match self.stt.nodes[v].ty\
    \ {\n            static_top_tree::SttNodeType::Compress => {\n               \
    \ (self.dp_up[v], self.dp_down[v]) = (Op::compress1(lu, ru), Op::compress2(ld,\
    \ rd));\n            }\n            static_top_tree::SttNodeType::Rake => {\n\
    \                (self.dp_up[v], self.dp_down[v]) = (Op::rake1(lu, ru), Op::rake2(ld,\
    \ ru));\n            }\n            _ => unreachable!(),\n        }\n    }\n}\n\
    \nimpl<Op: DynamicRerootingTreeDpOperator<Vertex = ()>> DynamicRerootingTreeDp<Op>\n\
    where\n    Op::Value: Clone,\n    Op::Edge: Clone,\n{\n    pub fn new(\n     \
    \   edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge + Clone],\n\
    \        root: usize,\n    ) -> Self {\n        let n = edges.len() + 1;\n   \
    \     DynamicRerootingTreeDp::with_vertices(edges, &vec![(); n], root)\n    }\n\
    }\n\n#[doc(hidden)]\npub trait Edge<Weight> {\n    fn weight(&self) -> Weight;\n\
    }\n\n#[doc(hidden)]\nimpl<Weight: Clone> Edge<Weight> for (usize, usize, Weight)\
    \ {\n    fn weight(&self) -> Weight {\n        self.2.clone()\n    }\n}\n\n#[doc(hidden)]\n\
    impl Edge<()> for (usize, usize) {\n    fn weight(&self) {}\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/static_top_tree/src/lib.rs
  isVerificationFile: false
  path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-04-22 05:57:06+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/point_set_tree_path_composite_sum/src/main.rs
documentation_of: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
- /library/crates/tree/dynamic_rerooting_tree_dp/src/lib.rs.html
title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
---
