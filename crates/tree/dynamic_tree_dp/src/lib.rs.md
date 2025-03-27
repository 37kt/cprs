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
    path: verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
    title: verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use heavy_light_decomposition::HeavyLightDecomposition;\nuse static_top_tree::StaticTopTree;\n\
    \npub trait DynamicTreeDpOperator {\n    type Value;\n    type Vertex;\n    type\
    \ Edge;\n\n    fn unit() -> Self::Value;\n    fn add_vertex(x: &Self::Value, v:\
    \ &Self::Vertex) -> Self::Value;\n\n    /// `x` \u306F\u5FC5\u305A\u5358\u4F53\
    \u306E\u9802\u70B9\u3067\u3042\u308B\u3053\u3068\u304C\u4FDD\u8A3C\u3055\u308C\
    \u3066\u3044\u308B  \n    /// (`rerooting_tree_dp` \u306E `add_edge` \u3068\u306F\
    \u7570\u306A\u308B\u5B9A\u7FA9)\n    fn add_edge(x: &Self::Value, e: &Self::Edge)\
    \ -> Self::Value;\n\n    /// `(a\u2190b], (a\u2190c] \u2192 (a\u2190b]`\n    fn\
    \ rake(l: &Self::Value, r: &Self::Value) -> Self::Value;\n\n    /// `(a\u2190\
    b], (b\u2190c] \u2192 (a\u2190c]`\n    fn compress(p: &Self::Value, c: &Self::Value)\
    \ -> Self::Value;\n}\n\npub struct DynamicTreeDp<Op: DynamicTreeDpOperator> {\n\
    \    hld: HeavyLightDecomposition,\n    stt: static_top_tree::StaticTopTree,\n\
    \    vertices: Vec<Op::Vertex>,\n    edges: Vec<Op::Edge>,\n    dp: Vec<Op::Value>,\n\
    }\n\nimpl<Op: DynamicTreeDpOperator> DynamicTreeDp<Op>\nwhere\n    Op::Value:\
    \ Clone,\n    Op::Vertex: Clone,\n    Op::Edge: Clone,\n{\n    pub fn with_vertices(\n\
    \        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge + Clone],\n\
    \        vertices: &[Op::Vertex],\n        root: usize,\n    ) -> Self {\n   \
    \     let n = vertices.len();\n        assert_eq!(n, edges.len() + 1);\n     \
    \   let hld = HeavyLightDecomposition::from_edges(edges, root);\n        let edges\
    \ = hld\n            .edges_order()\n            .map(|e| edges[e].weight())\n\
    \            .collect::<Vec<_>>();\n        let vertices = vertices.to_vec();\n\
    \        let stt = StaticTopTree::new(&hld);\n\n        let mut dp = vec![Op::unit();\
    \ n * 2 - 1];\n        for v in 0..n {\n            dp[v] = Op::add_vertex(&dp[v],\
    \ &vertices[v]);\n            if let Some(e) = hld.vertex_index(v).checked_sub(1)\
    \ {\n                dp[v] = Op::add_edge(&dp[v], &edges[e]);\n            }\n\
    \        }\n\n        let mut tree = Self {\n            hld,\n            stt,\n\
    \            vertices,\n            edges,\n            dp,\n        };\n    \
    \    for v in n..n * 2 - 1 {\n            tree.update(v);\n        }\n       \
    \ tree\n    }\n\n    pub fn set_vertex(&mut self, v: usize, x: Op::Vertex) {\n\
    \        self.vertices[v] = x;\n        self.dp[v] = Op::add_vertex(&Op::unit(),\
    \ &self.vertices[v]);\n        if let Some(e) = self.hld.vertex_index(v).checked_sub(1)\
    \ {\n            self.dp[v] = Op::add_edge(&self.dp[v], &self.edges[e]);\n   \
    \     }\n        let mut v = self.stt.nodes[v].par;\n        while v != !0 {\n\
    \            self.update(v);\n            v = self.stt.nodes[v].par;\n       \
    \ }\n    }\n\n    pub fn set_edge(&mut self, u: usize, v: usize, x: Op::Edge)\
    \ {\n        let e = self.hld.edge_index(u, v);\n        self.edges[e] = x;\n\
    \        let v = if self.hld.parent(u) == Some(v) { u } else { v };\n        self.dp[v]\
    \ = Op::add_edge(\n            &Op::add_vertex(&Op::unit(), &self.vertices[v]),\n\
    \            &self.edges[e],\n        );\n        let mut v = self.stt.nodes[v].par;\n\
    \        while v != !0 {\n            self.update(v);\n            v = self.stt.nodes[v].par;\n\
    \        }\n    }\n\n    pub fn fold(&self) -> Op::Value {\n        self.dp.last().unwrap().clone()\n\
    \    }\n\n    fn update(&mut self, v: usize) {\n        let l = &self.dp[self.stt.nodes[v].lch];\n\
    \        let r = &self.dp[self.stt.nodes[v].rch];\n        match self.stt.nodes[v].ty\
    \ {\n            static_top_tree::SttNodeType::Compress => {\n               \
    \ self.dp[v] = Op::compress(l, r);\n            }\n            static_top_tree::SttNodeType::Rake\
    \ => {\n                self.dp[v] = Op::rake(l, r);\n            }\n        \
    \    _ => unreachable!(),\n        }\n    }\n}\n\nimpl<Op: DynamicTreeDpOperator<Vertex\
    \ = ()>> DynamicTreeDp<Op>\nwhere\n    Op::Value: Clone,\n    Op::Edge: Clone,\n\
    {\n    pub fn new(\n        edges: &[impl Edge<Op::Edge> + heavy_light_decomposition::Edge\
    \ + Clone],\n        root: usize,\n    ) -> Self {\n        let n = edges.len()\
    \ + 1;\n        DynamicTreeDp::with_vertices(edges, &vec![(); n], root)\n    }\n\
    }\n\n#[doc(hidden)]\npub trait Edge<Weight> {\n    fn weight(&self) -> Weight;\n\
    }\n\n#[doc(hidden)]\nimpl<Weight: Clone> Edge<Weight> for (usize, usize, Weight)\
    \ {\n    fn weight(&self) -> Weight {\n        self.2.clone()\n    }\n}\n\n#[doc(hidden)]\n\
    impl Edge<()> for (usize, usize) {\n    fn weight(&self) -> () {\n        ()\n\
    \    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  - crates/tree/static_top_tree/src/lib.rs
  isVerificationFile: false
  path: crates/tree/dynamic_tree_dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-27 07:31:57+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/point_set_tree_path_composite_sum_fixed_root/src/main.rs
documentation_of: crates/tree/dynamic_tree_dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/dynamic_tree_dp/src/lib.rs
- /library/crates/tree/dynamic_tree_dp/src/lib.rs.html
title: crates/tree/dynamic_tree_dp/src/lib.rs
---
