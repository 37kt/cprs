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
    path: crates/graph/graph/src/builder.rs
    title: crates/graph/graph/src/builder.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/edge.rs
    title: crates/graph/graph/src/edge.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
    title: verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
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
  code: "use csr_array::CsrArray;\nuse graph::Edge;\n\n#[derive(Debug)]\npub struct\
    \ CentroidDecomposition {\n    pub root: usize,\n    pub vs: [Vec<usize>; 2],\n\
    \    pub par: [Vec<usize>; 2],\n}\n\nimpl CentroidDecomposition {\n    pub fn\
    \ solve<W>(g: &CsrArray<impl Edge<W>>, mut f: impl FnMut(&CentroidDecomposition))\
    \ {\n        let mut cd = CdImpl::new(g);\n        cd.dfs_decompose(0, &mut f);\n\
    \    }\n\n    fn new() -> Self {\n        Self {\n            root: !0,\n    \
    \        vs: [vec![], vec![]],\n            par: [vec![], vec![]],\n        }\n\
    \    }\n\n    fn clear(&mut self) {\n        self.root = !0;\n        for i in\
    \ 0..2 {\n            self.vs[i].clear();\n            self.par[i].clear();\n\
    \        }\n    }\n}\n\nstruct CdImpl {\n    edges: Vec<usize>,\n    head: Vec<usize>,\n\
    \    tail: Vec<usize>,\n\n    sz: Vec<usize>,\n    cd: CentroidDecomposition,\n\
    }\n\nimpl CdImpl {\n    fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {\n    \
    \    let n = g.len();\n        let mut head = vec![0; n];\n        let mut tail\
    \ = vec![0; n];\n        for i in 0..n {\n            tail[i] = g[i].len();\n\
    \            if i > 0 {\n                tail[i] += tail[i - 1];\n           \
    \     head[i] = tail[i - 1];\n            }\n        }\n        let edges = g.iter().flatten().map(|e|\
    \ e.to()).collect();\n        Self {\n            edges,\n            head,\n\
    \            tail,\n            sz: vec![0; n],\n            cd: CentroidDecomposition::new(),\n\
    \        }\n    }\n\n    fn dfs_sz(&mut self, v: usize, p: usize) {\n        self.sz[v]\
    \ = 1;\n        for i in self.head[v]..self.tail[v] {\n            let u = self.edges[i];\n\
    \            if u != p {\n                self.dfs_sz(u, v);\n               \
    \ self.sz[v] += self.sz[u];\n            }\n        }\n    }\n\n    fn dfs_centroid(&mut\
    \ self, v: usize, p: usize, mid: usize) -> usize {\n        for i in self.head[v]..self.tail[v]\
    \ {\n            let u = self.edges[i];\n            if u != p && self.sz[u] >\
    \ mid {\n                return self.dfs_centroid(u, v, mid);\n            }\n\
    \        }\n        v\n    }\n\n    fn dfs_build_tree(&mut self, color: usize,\
    \ v: usize, p: usize) {\n        self.cd.vs[color].push(v);\n        self.cd.par[color].push(p);\n\
    \        for i in self.head[v]..self.tail[v] {\n            let u = self.edges[i];\n\
    \            if u != p {\n                self.dfs_build_tree(color, u, v);\n\
    \            }\n        }\n    }\n\n    fn dfs_decompose(&mut self, v: usize,\
    \ f: &mut impl FnMut(&CentroidDecomposition)) {\n        self.dfs_sz(v, !0);\n\
    \        if self.sz[v] <= 2 {\n            return;\n        }\n        let c =\
    \ self.dfs_centroid(v, !0, self.sz[v] / 2);\n        self.dfs_sz(c, !0);\n\n \
    \       let mut sum_sz = 0;\n        let mut m = self.head[c];\n        for i\
    \ in self.head[c]..self.tail[c] {\n            let v = self.edges[i];\n      \
    \      if sum_sz + self.sz[v] <= (self.sz[c] - 1) / 2 {\n                sum_sz\
    \ += self.sz[v];\n                self.edges.swap(m, i);\n                m +=\
    \ 1;\n            }\n        }\n\n        self.cd.clear();\n        self.cd.root\
    \ = c;\n        for i in self.head[c]..self.tail[c] {\n            let v = self.edges[i];\n\
    \            let color = if i < m { 0 } else { 1 };\n            self.dfs_build_tree(color,\
    \ v, c);\n        }\n        f(&self.cd);\n\n        std::mem::swap(&mut self.head[c],\
    \ &mut m);\n        self.dfs_decompose(c, f);\n        std::mem::swap(&mut self.head[c],\
    \ &mut m);\n        std::mem::swap(&mut self.tail[c], &mut m);\n        self.dfs_decompose(c,\
    \ f);\n        std::mem::swap(&mut self.tail[c], &mut m);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/centroid_decomposition/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-27 00:50:57+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/frequency_table_of_tree_distance/src/main.rs
documentation_of: crates/tree/centroid_decomposition/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/centroid_decomposition/src/lib.rs
- /library/crates/tree/centroid_decomposition/src/lib.rs.html
title: crates/tree/centroid_decomposition/src/lib.rs
---
