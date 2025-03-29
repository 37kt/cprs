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
  _extendedRequiredBy:
  - icon: ':warning:'
    path: crates/tree/tree_contour_range/src/lib.rs
    title: crates/tree/tree_contour_range/src/lib.rs
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
  code: "use csr_array::CsrArray;\nuse graph::Edge;\n\n#[derive(Debug, Clone, Copy)]\n\
    pub struct CentroidDecomposition<'a> {\n    pub root: usize,\n    pub vs: &'a\
    \ [usize],\n    pub mid: usize,\n    pub par: &'a [usize],\n}\n\nimpl<'a> CentroidDecomposition<'a>\
    \ {\n    pub fn solve<W>(g: &CsrArray<impl Edge<W>>, mut f: impl FnMut(CentroidDecomposition))\
    \ {\n        let mut cd = CdImpl::new(g);\n        cd.dfs_decompose(0, &mut f);\n\
    \    }\n}\n\nstruct CdImpl {\n    edges: Vec<usize>,\n    head: Vec<usize>,\n\
    \    tail: Vec<usize>,\n\n    sz: Vec<usize>,\n    par: Vec<usize>,\n}\n\nimpl\
    \ CdImpl {\n    fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {\n        let n\
    \ = g.len();\n        let mut head = vec![0; n];\n        let mut tail = vec![0;\
    \ n];\n        for i in 0..n {\n            tail[i] = g[i].len();\n          \
    \  if i > 0 {\n                tail[i] += tail[i - 1];\n                head[i]\
    \ = tail[i - 1];\n            }\n        }\n        let edges = g.iter().flatten().map(|e|\
    \ e.to()).collect();\n        Self {\n            edges,\n            head,\n\
    \            tail,\n            sz: vec![0; n],\n            par: vec![!0; n],\n\
    \        }\n    }\n\n    fn dfs_decompose(&mut self, v: usize, f: &mut impl FnMut(CentroidDecomposition))\
    \ {\n        self.par[v] = !0;\n        let mut ord = vec![v];\n        let mut\
    \ i = 0;\n        while i < ord.len() {\n            let v = ord[i];\n       \
    \     self.sz[v] = 1;\n            for &u in &self.edges[self.head[v]..self.tail[v]]\
    \ {\n                if u != self.par[v] {\n                    self.par[u] =\
    \ v;\n                    ord.push(u);\n                }\n            }\n   \
    \         i += 1;\n        }\n\n        let n = ord.len();\n        if n <= 2\
    \ {\n            return;\n        }\n\n        let mut c = !0;\n        for &v\
    \ in ord.iter().rev() {\n            if self.par[v] != !0 {\n                self.sz[self.par[v]]\
    \ += self.sz[v];\n            }\n            if c == !0 && self.sz[v] >= (n +\
    \ 1) / 2 {\n                c = v;\n            }\n        }\n\n        let mut\
    \ p = !0;\n        let mut p_sz = 0;\n        let mut v = c;\n        while v\
    \ != !0 {\n            std::mem::swap(&mut self.sz[v], &mut p_sz);\n         \
    \   self.sz[v] = n - self.sz[v];\n            std::mem::swap(&mut self.par[v],\
    \ &mut p);\n            std::mem::swap(&mut p, &mut v);\n        }\n\n       \
    \ let mut sum_sz = 0;\n        let mut m = self.head[c];\n        for i in self.head[c]..self.tail[c]\
    \ {\n            let v = self.edges[i];\n            if sum_sz + self.sz[v] <=\
    \ (self.sz[c] - 1) / 2 {\n                sum_sz += self.sz[v];\n            \
    \    self.edges.swap(m, i);\n                m += 1;\n            }\n        }\n\
    \n        let mut vs = Vec::with_capacity(n - 1);\n        let mut mid = 0;\n\
    \        for (color, cs) in [\n            (0, &self.edges[self.head[c]..m]),\n\
    \            (1, &self.edges[m..self.tail[c]]),\n        ] {\n            for\
    \ &v in cs {\n                vs.push(v);\n            }\n            let mut\
    \ i = mid;\n            while i < vs.len() {\n                let v = vs[i];\n\
    \                for &u in &self.edges[self.head[v]..self.tail[v]] {\n       \
    \             if u != self.par[v] {\n                        vs.push(u);\n   \
    \                 }\n                }\n                i += 1;\n            }\n\
    \            if color == 0 {\n                mid = vs.len();\n            }\n\
    \        }\n        f(CentroidDecomposition {\n            root: c,\n        \
    \    vs: &vs,\n            mid,\n            par: &self.par,\n        });\n\n\
    \        std::mem::swap(&mut self.head[c], &mut m);\n        self.dfs_decompose(c,\
    \ f);\n        std::mem::swap(&mut self.head[c], &mut m);\n        std::mem::swap(&mut\
    \ self.tail[c], &mut m);\n        self.dfs_decompose(c, f);\n        std::mem::swap(&mut\
    \ self.tail[c], &mut m);\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/graph/graph/src/builder.rs
  - crates/graph/graph/src/edge.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/centroid_decomposition/src/lib.rs
  requiredBy:
  - crates/tree/tree_contour_range/src/lib.rs
  timestamp: '2025-03-28 02:27:58+00:00'
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
