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
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
    title: crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/dynamic_tree_dp/src/lib.rs
    title: crates/tree/dynamic_tree_dp/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Reverse, collections::BinaryHeap};\n\nuse csr_array::CsrArray;\n\
    use heavy_light_decomposition::HeavyLightDecomposition;\n\n#[derive(Debug, Clone,\
    \ Copy, PartialEq, Eq)]\npub enum SttNodeType {\n    Value,\n    Rake,\n    Compress,\n\
    }\n\n#[derive(Debug, Clone, Copy)]\npub struct SttNode {\n    pub ty: SttNodeType,\n\
    \    pub par: usize,\n    pub lch: usize,\n    pub rch: usize,\n}\n\n#[derive(Clone)]\n\
    pub struct StaticTopTree {\n    pub nodes: Vec<SttNode>,\n}\n\nimpl StaticTopTree\
    \ {\n    pub fn new(hld: &HeavyLightDecomposition) -> Self {\n        let n =\
    \ hld.len();\n        let root = hld.root();\n        let mut nodes = Vec::with_capacity(n\
    \ * 2 - 1);\n        nodes.extend(\n            std::iter::repeat(SttNode {\n\
    \                ty: SttNodeType::Value,\n                par: !0,\n         \
    \       lch: !0,\n                rch: !0,\n            })\n            .take(n),\n\
    \        );\n        let mut stt = Self { nodes };\n        let children = hld.children();\n\
    \        stt.dfs_build(root, hld, &children);\n        stt\n    }\n\n    fn dfs_build(\n\
    \        &mut self,\n        v: usize,\n        hld: &HeavyLightDecomposition,\n\
    \        children: &CsrArray<usize>,\n    ) -> (usize, usize) {\n        assert!(hld.head(v)\
    \ == v);\n\n        let path = {\n            let mut path = vec![v];\n      \
    \      let mut v = v;\n            while children[v].len() > 0 {\n           \
    \     v = children[v][0];\n                path.push(v);\n            }\n    \
    \        path\n        };\n\n        let mut stack = vec![(0, v)];\n        for\
    \ i in 1..path.len() {\n            let mut pq = BinaryHeap::new();\n        \
    \    let mut v = path[i];\n            pq.push(Reverse((0, v)));\n           \
    \ for &c in children[path[i - 1]].iter().skip(1) {\n                pq.push(Reverse(self.dfs_build(c,\
    \ hld, children)));\n            }\n            while pq.len() >= 2 {\n      \
    \          let Reverse((h1, mut v1)) = pq.pop().unwrap();\n                let\
    \ Reverse((h2, mut v2)) = pq.pop().unwrap();\n                if v2 == v {\n \
    \                   std::mem::swap(&mut v1, &mut v2);\n                }\n   \
    \             let v3 = self.new_node(SttNodeType::Rake, v1, v2);\n           \
    \     if v == v1 {\n                    v = v3;\n                }\n         \
    \       pq.push(Reverse((h1.max(h2) + 1, v3)));\n            }\n            stack.push(pq.pop().unwrap().0);\n\
    \n            loop {\n                if matches!(&stack[..], &[.., (h0, _), (h1,\
    \ _), (h2, _)] if (h0 == h1 || h0 <= h2))\n                {\n               \
    \     let x = stack.pop().unwrap();\n                    self.compress_last_two(&mut\
    \ stack);\n                    stack.push(x);\n                } else if matches!(&stack[..],\
    \ &[.., (h0, _), (h1, _)] if h0 <= h1) {\n                    self.compress_last_two(&mut\
    \ stack);\n                } else {\n                    break;\n            \
    \    }\n            }\n        }\n\n        while stack.len() >= 2 {\n       \
    \     self.compress_last_two(&mut stack);\n        }\n        stack.pop().unwrap()\n\
    \    }\n\n    fn new_node(&mut self, ty: SttNodeType, lch: usize, rch: usize)\
    \ -> usize {\n        let v = self.nodes.len();\n        self.nodes.push(SttNode\
    \ {\n            ty,\n            par: !0,\n            lch,\n            rch,\n\
    \        });\n        self.nodes[lch].par = v;\n        self.nodes[rch].par =\
    \ v;\n        v\n    }\n\n    fn compress_last_two(&mut self, stack: &mut Vec<(usize,\
    \ usize)>) {\n        let (h2, v2) = stack.pop().unwrap();\n        let (h1, v1)\
    \ = stack.pop().unwrap();\n        stack.push((h1.max(h2) + 1, self.new_node(SttNodeType::Compress,\
    \ v1, v2)));\n    }\n}\n"
  dependsOn:
  - crates/data_structure/csr_array/src/builder.rs
  - crates/data_structure/csr_array/src/csr_array.rs
  - crates/data_structure/csr_array/src/lib.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/static_top_tree/src/lib.rs
  requiredBy:
  - crates/tree/dynamic_rerooting_tree_dp/src/lib.rs
  - crates/tree/dynamic_tree_dp/src/lib.rs
  timestamp: '2025-03-29 09:22:56+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/static_top_tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/static_top_tree/src/lib.rs
- /library/crates/tree/static_top_tree/src/lib.rs.html
title: crates/tree/static_top_tree/src/lib.rs
---
