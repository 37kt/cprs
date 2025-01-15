---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/data-structure/csr-array/src/lib.rs
    title: crates/data-structure/csr-array/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy-light-decomposition/src/lib.rs
    title: crates/tree/heavy-light-decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{cmp::Reverse, collections::BinaryHeap};\n\nuse csr_array::CSRArray;\n\
    use graph::UndirectedGraph;\nuse heavy_light_decomposition::HeavyLightDecomposition;\n\
    \npub struct StaticTopTree {\n    pub hld: HeavyLightDecomposition,\n    pub par:\
    \ Vec<usize>,\n    pub lch: Vec<usize>,\n    pub rch: Vec<usize>,\n    pub top:\
    \ Vec<usize>,\n    pub bottom: Vec<usize>,\n    pub is_compress: Vec<bool>,\n\
    }\n\nimpl StaticTopTree {\n    pub fn new<V: Clone, E: Clone>(g: &UndirectedGraph<V,\
    \ E>, root: usize) -> Self {\n        let n = g.len();\n        let mut stt =\
    \ Self {\n            hld: HeavyLightDecomposition::new(g, root),\n          \
    \  par: Vec::with_capacity(n * 2 - 1),\n            lch: Vec::with_capacity(n\
    \ * 2 - 1),\n            rch: Vec::with_capacity(n * 2 - 1),\n            top:\
    \ Vec::with_capacity(n * 2 - 1),\n            bottom: Vec::with_capacity(n * 2\
    \ - 1),\n            is_compress: Vec::with_capacity(n * 2 - 1),\n        };\n\
    \        for v in 0..n {\n            stt.par.push(!0);\n            stt.lch.push(!0);\n\
    \            stt.rch.push(!0);\n            stt.top.push(stt.hld.parent(v));\n\
    \            stt.bottom.push(v);\n            stt.is_compress.push(false);\n \
    \       }\n        let children = stt.hld.children();\n        stt.build_dfs(root,\
    \ &children);\n        assert!(stt.par.len() == n * 2 - 1);\n        stt\n   \
    \ }\n\n    // (height, index)\n    fn build_dfs(&mut self, v: usize, children:\
    \ &CSRArray<usize>) -> (usize, usize) {\n        assert!(self.hld.leader(v) ==\
    \ v);\n\n        let path = {\n            let mut path = vec![v];\n         \
    \   let mut v = v;\n            while children[v].len() > 0 {\n              \
    \  v = children[v][0];\n                path.push(v);\n            }\n       \
    \     path\n        };\n        let mut stack = vec![(0, v)];\n        for i in\
    \ 1..path.len() {\n            let mut pq = BinaryHeap::new();\n            let\
    \ mut v = path[i];\n            pq.push(Reverse((0, v)));\n            for &c\
    \ in children[path[i - 1]].iter().skip(1) {\n                pq.push(Reverse(self.build_dfs(c,\
    \ children)));\n            }\n            while pq.len() >= 2 {\n           \
    \     let Reverse((h1, mut v1)) = pq.pop().unwrap();\n                let Reverse((h2,\
    \ mut v2)) = pq.pop().unwrap();\n                if v2 == v {\n              \
    \      std::mem::swap(&mut v1, &mut v2);\n                }\n                let\
    \ v3 = self.new_node(v1, v2, self.top[v1], self.bottom[v2], false);\n        \
    \        if v == v1 {\n                    v = v3;\n                }\n      \
    \          pq.push(Reverse((h1.max(h2) + 1, v3)));\n            }\n          \
    \  stack.push(pq.pop().unwrap().0);\n\n            loop {\n                let\
    \ n = stack.len();\n                if n >= 3 && (stack[n - 3].0 == stack[n -\
    \ 2].0 || stack[n - 3].0 <= stack[n - 1].0)\n                {\n             \
    \       let hv = stack.pop().unwrap();\n                    self.merge(&mut stack);\n\
    \                    stack.push(hv);\n                } else if n >= 2 && stack[n\
    \ - 2].0 <= stack[n - 1].0 {\n                    self.merge(&mut stack);\n  \
    \              } else {\n                    break;\n                }\n     \
    \       }\n        }\n\n        while stack.len() >= 2 {\n            self.merge(&mut\
    \ stack);\n        }\n        stack.pop().unwrap()\n    }\n\n    fn new_node(\n\
    \        &mut self,\n        lch: usize,\n        rch: usize,\n        top: usize,\n\
    \        bottom: usize,\n        is_compress: bool,\n    ) -> usize {\n      \
    \  let v = self.par.len();\n        self.par[lch] = v;\n        self.par[rch]\
    \ = v;\n        self.par.push(!0);\n        self.lch.push(lch);\n        self.rch.push(rch);\n\
    \        self.top.push(top);\n        self.bottom.push(bottom);\n        self.is_compress.push(is_compress);\n\
    \        v\n    }\n\n    fn merge(&mut self, stack: &mut Vec<(usize, usize)>)\
    \ {\n        let (h2, v2) = stack.pop().unwrap();\n        let (h1, v1) = stack.pop().unwrap();\n\
    \        stack.push((\n            h1.max(h2) + 1,\n            self.new_node(v1,\
    \ v2, self.top[v1], self.bottom[v2], true),\n        ));\n    }\n}\n"
  dependsOn:
  - crates/data-structure/csr-array/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/tree/heavy-light-decomposition/src/lib.rs
  isVerificationFile: false
  path: crates/tree/static-top-tree/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-15 06:25:46+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/static-top-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/static-top-tree/src/lib.rs
- /library/crates/tree/static-top-tree/src/lib.rs.html
title: crates/tree/static-top-tree/src/lib.rs
---