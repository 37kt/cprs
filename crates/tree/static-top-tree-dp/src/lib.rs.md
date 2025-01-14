---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
    title: verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::UndirectedGraph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    \n#[derive(Clone, Copy)]\nenum Type {\n    Vertex,\n    Compress,\n    Rake,\n\
    \    AddEdge,\n    AddVertex,\n}\n\n/// Static Top Tree (Fixed root)  \n/// 0\
    \ \u3092\u6839\u3068\u3057\u3066\u3044\u308B\npub struct StaticTopTree {\n   \
    \ stt_root: usize,\n    par: Vec<usize>,\n    lch: Vec<usize>,\n    rch: Vec<usize>,\n\
    \    ty: Vec<Type>,\n    edge: Vec<usize>,\n    par_edge: Vec<usize>,\n    child:\
    \ Vec<usize>,\n    cnt: usize,\n    hld: HeavyLightDecomposition,\n}\n\nimpl StaticTopTree\
    \ {\n    /// 0 \u3092\u6839\u3068\u3059\u308B Static Top Tree \u3092\u69CB\u7BC9\
    \u3059\u308B\n    pub fn new<V: Clone, E: Clone>(g: &UndirectedGraph<V, E>) ->\
    \ Self {\n        let n = g.len();\n        let mut s = Self {\n            stt_root:\
    \ !0,\n            par: vec![!0; n * 4],\n            lch: vec![!0; n * 4],\n\
    \            rch: vec![!0; n * 4],\n            ty: vec![Type::Vertex; n * 4],\n\
    \            edge: vec![!0; n * 4],\n            par_edge: vec![!0; n],\n    \
    \        child: vec![!0; n - 1],\n            cnt: n,\n            hld: HeavyLightDecomposition::new(g,\
    \ 0),\n        };\n        let dist = s.hld.dist_table(0);\n        for v in 0..n\
    \ {\n            for &(u, _) in &g[v] {\n                let e = s.hld.edge_index(u,\
    \ v) - 1;\n                if dist[v] < dist[u] {\n                    s.par_edge[u]\
    \ = e;\n                    s.child[e] = u;\n                }\n            }\n\
    \        }\n        s.stt_root = s.compress(0, g).0;\n        s\n    }\n\n   \
    \ /// \u9802\u70B9\u6570\u3092\u8FD4\u3059\n    pub fn len(&self) -> usize {\n\
    \        self.cnt\n    }\n\n    fn add(&mut self, mut k: usize, l: usize, r: usize,\
    \ t: Type) -> usize {\n        if k == !0 {\n            k = self.cnt;\n     \
    \       self.cnt += 1;\n        }\n        self.par[k] = !0;\n        self.lch[k]\
    \ = l;\n        self.rch[k] = r;\n        self.ty[k] = t;\n        if l != !0\
    \ {\n            self.par[l] = k;\n        }\n        if r != !0 {\n         \
    \   self.par[r] = k;\n        }\n        k\n    }\n\n    fn merge(&mut self, a:\
    \ &[(usize, usize)], t: Type) -> (usize, usize) {\n        if a.len() == 1 {\n\
    \            return a[0];\n        }\n        let mut u = a.iter().map(|&(_, s)|\
    \ s).sum::<usize>();\n        let mut m = 0;\n        while m < a.len() && a[m].1\
    \ < u {\n            u -= u.min(a[m].1 * 2);\n            m += 1;\n        }\n\
    \        let (i, si) = self.merge(&a[..m], t);\n        let (j, sj) = self.merge(&a[m..],\
    \ t);\n        let res = (self.add(!0, i, j, t), si + sj);\n        match t {\n\
    \            Type::Compress => {\n                self.edge[res.0] = self.par_edge[a[m].0];\n\
    \            }\n            _ => (),\n        }\n        res\n    }\n\n    fn\
    \ compress<V: Clone, E: Clone>(\n        &mut self,\n        mut i: usize,\n \
    \       g: &UndirectedGraph<V, E>,\n    ) -> (usize, usize) {\n        let mut\
    \ chs = vec![self.add_vertex(i, g)];\n        while self.hld.heavy_child(i) !=\
    \ !0 {\n            i = self.hld.heavy_child(i);\n            chs.push(self.add_vertex(i,\
    \ g));\n        }\n        self.merge(&chs, Type::Compress)\n    }\n\n    fn rake<V:\
    \ Clone, E: Clone>(&mut self, i: usize, g: &UndirectedGraph<V, E>) -> (usize,\
    \ usize) {\n        let mut chs = vec![];\n        for &(u, _) in &g[i] {\n  \
    \          if u == self.hld.parent(i) || u == self.hld.heavy_child(i) {\n    \
    \            continue;\n            }\n            chs.push(self.add_edge(u, g));\n\
    \        }\n        if chs.is_empty() {\n            (!0, 0)\n        } else {\n\
    \            self.merge(&chs, Type::Rake)\n        }\n    }\n\n    fn add_edge<V:\
    \ Clone, E: Clone>(\n        &mut self,\n        i: usize,\n        g: &UndirectedGraph<V,\
    \ E>,\n    ) -> (usize, usize) {\n        let (j, sj) = self.compress(i, g);\n\
    \        let res = (self.add(!0, j, !0, Type::AddEdge), sj);\n        self.edge[res.0]\
    \ = self.par_edge[i];\n        res\n    }\n\n    fn add_vertex<V: Clone, E: Clone>(\n\
    \        &mut self,\n        i: usize,\n        g: &UndirectedGraph<V, E>,\n \
    \   ) -> (usize, usize) {\n        let (j, sj) = self.rake(i, g);\n        (\n\
    \            self.add(\n                i,\n                j,\n             \
    \   !0,\n                if j == !0 {\n                    Type::Vertex\n    \
    \            } else {\n                    Type::AddVertex\n                },\n\
    \            ),\n            sj + 1,\n        )\n    }\n}\n\n#[derive(Clone)]\n\
    enum Data<Path, Point> {\n    Path(Path),\n    Point(Point),\n}\n\nimpl<Path,\
    \ Point> Data<Path, Point> {\n    fn unwrap_path(&self) -> &Path {\n        match\
    \ self {\n            Data::Path(p) => p,\n            _ => panic!(),\n      \
    \  }\n    }\n\n    fn unwrap_point(&self) -> &Point {\n        match self {\n\
    \            Data::Point(p) => p,\n            _ => panic!(),\n        }\n   \
    \ }\n}\n\n/// \u6728 dp \u306E\u5404\u7A2E\u6F14\u7B97\u3092\u5B9A\u7FA9\u3059\
    \u308B  \npub trait TreeDPOperator {\n    type Path: Clone;\n    type Point: Clone;\n\
    \    type V: Clone;\n    type E: Clone;\n\n    /// \u9802\u70B9 v \u306E\u307F\
    \u304B\u3089\u306A\u308B Path Cluster \u3092\u751F\u6210\u3059\u308B\n    fn vertex(v:\
    \ &Self::V) -> Self::Path;\n\n    /// 2 \u3064\u306E Path Cluster \u3092 Heavy\
    \ Edge \u3067\u7E4B\u3044\u3067 1 \u3064\u306E Path Cluster \u306B\u3059\u308B\
    \n    fn compress(p: &Self::Path, c: &Self::Path, e: &Self::E) -> Self::Path;\n\
    \n    /// 2 \u3064\u306E Point Cluster \u306E virtual \u306A\u6839\u3092\u5408\
    \u4F53\u3055\u305B\u3066 1 \u3064\u306E Point Cluster \u306B\u3059\u308B\n   \
    \ fn rake(l: &Self::Point, r: &Self::Point) -> Self::Point;\n\n    /// Path Cluster\
    \ \u306E\u6839\u3068 virtual \u306A\u9802\u70B9\u3092 Light Edge \u3067\u7E4B\u3044\
    \u3067 Point Cluster \u306B\u3059\u308B\n    fn add_edge(d: &Self::Path, e: &Self::E)\
    \ -> Self::Point;\n\n    /// Point Cluster \u306E\u6839\u306E virtual \u306A\u9802\
    \u70B9\u3092 v \u306B\u7F6E\u304D\u63DB\u3048\u3066 Path Cluster \u306B\u3059\u308B\
    \n    fn add_vertex(d: &Self::Point, v: &Self::V) -> Self::Path;\n}\n\npub struct\
    \ StaticTopTreeDP<O: TreeDPOperator> {\n    stt: StaticTopTree,\n    sum: Vec<Data<O::Path,\
    \ O::Point>>,\n    vertex: Vec<O::V>,\n    edge: Vec<O::E>,\n    op: std::marker::PhantomData<O>,\n\
    }\n\nimpl<O: TreeDPOperator> StaticTopTreeDP<O> {\n    /// 0 \u3092\u6839\u3068\
    \u3059\u308B Static Top Tree \u3092\u69CB\u7BC9\u3059\u308B\n    pub fn new(g:\
    \ &UndirectedGraph<O::V, O::E>) -> Self {\n        let stt = StaticTopTree::new(g);\n\
    \        let mut sum = vec![Data::Path(O::vertex(&g.vertex(0))); stt.len()];\n\
    \        let vertex = (0..g.len())\n            .map(|v| g.vertex(v).clone())\n\
    \            .collect::<Vec<_>>();\n        let mut edge = if g.len() == 1 {\n\
    \            vec![]\n        } else {\n            vec![g[0][0].1.clone(); g.len()\
    \ - 1]\n        };\n        for v in 0..g.len() {\n            sum[v] = Data::Path(O::vertex(&g.vertex(v)));\n\
    \            for (u, w) in &g[v] {\n                let e = stt.hld.edge_index(v,\
    \ *u) - 1;\n                edge[e] = w.clone();\n            }\n        }\n \
    \       let mut s = Self {\n            stt,\n            sum,\n            vertex,\n\
    \            edge,\n            op: std::marker::PhantomData,\n        };\n  \
    \      s.dfs(s.stt.stt_root);\n        s\n    }\n\n    /// 0 \u3092\u6839\u3068\
    \u3057\u305F\u3068\u304D\u306E dp \u306E\u5024\u3092\u8FD4\u3059\n    pub fn prod(&self)\
    \ -> O::Path {\n        self.sum[self.stt.stt_root].unwrap_path().clone()\n  \
    \  }\n\n    /// \u9802\u70B9 v \u306E\u5024\u3092 x \u306B\u66F4\u65B0\u3059\u308B\
    \n    pub fn set_vertex(&mut self, mut v: usize, x: O::V) {\n        self.vertex[v]\
    \ = x.clone();\n        while v != !0 {\n            self.update(v);\n       \
    \     v = self.stt.par[v];\n        }\n    }\n\n    /// \u8FBA (u, v) \u306E\u5024\
    \u3092 x \u306B\u66F4\u65B0\u3059\u308B\n    pub fn set_edge(&mut self, u: usize,\
    \ v: usize, x: O::E) {\n        let e = self.stt.hld.edge_index(u, v) - 1;\n \
    \       self.edge[e] = x.clone();\n        let mut v = self.stt.child[e];\n  \
    \      while v != !0 {\n            self.update(v);\n            v = self.stt.par[v];\n\
    \        }\n    }\n\n    fn dfs(&mut self, v: usize) {\n        if self.stt.lch[v]\
    \ != !0 {\n            self.dfs(self.stt.lch[v]);\n        }\n        if self.stt.rch[v]\
    \ != !0 {\n            self.dfs(self.stt.rch[v]);\n        }\n        self.update(v);\n\
    \    }\n\n    fn update(&mut self, v: usize) {\n        match self.stt.ty[v] {\n\
    \            Type::Vertex => {\n                self.sum[v] = Data::Path(O::vertex(&self.vertex[v]));\n\
    \            }\n            Type::Compress => {\n                self.sum[v] =\
    \ Data::Path(O::compress(\n                    self.sum[self.stt.lch[v]].unwrap_path(),\n\
    \                    self.sum[self.stt.rch[v]].unwrap_path(),\n              \
    \      &self.edge[self.stt.edge[v]],\n                ));\n            }\n   \
    \         Type::Rake => {\n                self.sum[v] = Data::Point(O::rake(\n\
    \                    self.sum[self.stt.lch[v]].unwrap_point(),\n             \
    \       self.sum[self.stt.rch[v]].unwrap_point(),\n                ));\n     \
    \       }\n            Type::AddEdge => {\n                self.sum[v] = Data::Point(O::add_edge(\n\
    \                    self.sum[self.stt.lch[v]].unwrap_path(),\n              \
    \      &self.edge[self.stt.edge[v]],\n                ));\n            }\n   \
    \         Type::AddVertex => {\n                self.sum[v] = Data::Path(O::add_vertex(\n\
    \                    self.sum[self.stt.lch[v]].unwrap_point(),\n             \
    \       &self.vertex[v],\n                ));\n            }\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/static-top-tree-dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 06:20:21+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/point_set_tree_path_composite_sum_fixed_root/src/main.rs
documentation_of: crates/tree/static-top-tree-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/static-top-tree-dp/src/lib.rs
- /library/crates/tree/static-top-tree-dp/src/lib.rs.html
title: crates/tree/static-top-tree-dp/src/lib.rs
---
