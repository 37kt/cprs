---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links:
    - https://tjkendev.github.io/procon-library/cpp/graph/auxiliary_tree.html
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\n\n// Reference:\n// https://tjkendev.github.io/procon-library/cpp/graph/auxiliary_tree.html\n\
    \npub struct CompressedTree {\n    fs: Vec<usize>,\n    ls: Vec<usize>,\n    depth:\
    \ Vec<usize>,\n    st: Vec<Vec<usize>>,\n    lg: Vec<usize>,\n    idx: Vec<usize>,\n\
    }\n\nimpl CompressedTree {\n    pub fn new(g: &Graph<(), ()>) -> Self {\n    \
    \    let n = g.len();\n        let mut c = Self {\n            fs: vec![0; n],\n\
    \            ls: vec![0; n],\n            depth: vec![0; n],\n            st:\
    \ vec![vec![]],\n            lg: vec![],\n            idx: vec![0; n],\n     \
    \   };\n\n        c.ett_dfs(0, !0, 0, g);\n\n        c.lg = vec![0; 2];\n    \
    \    let m = c.st[0].len();\n        for i in 2..=m {\n            c.lg.push(c.lg[i\
    \ >> 1] + 1);\n        }\n\n        for i in 0..c.lg[m] {\n            c.st.push(vec![]);\n\
    \            let b = 1 << i;\n            for j in 0..=m - (b << 1) {\n      \
    \          let t = if c.depth[c.st[i][j]] <= c.depth[c.st[i][j + b]] {\n     \
    \               c.st[i][j]\n                } else {\n                    c.st[i][j\
    \ + b]\n                };\n                c.st[i + 1].push(t);\n           \
    \ }\n        }\n\n        c\n    }\n\n    pub fn build(&mut self, vs: &[usize])\
    \ -> (Graph<(), ()>, Vec<usize>) {\n        let mut vs = vs.to_vec();\n      \
    \  vs.sort_by_key(|&v| self.fs[v]);\n        for i in 0..vs.len() - 1 {\n    \
    \        vs.push(self.lca(vs[i], vs[i + 1]));\n        }\n        vs.sort_by_key(|&v|\
    \ self.fs[v]);\n        vs.dedup();\n        let mut stk = vec![];\n        let\
    \ mut es = vec![];\n        let mut idx = vec![0; vs.len()];\n        for i in\
    \ 0..vs.len() {\n            self.idx[vs[i]] = i;\n            idx[i] = vs[i];\n\
    \        }\n        for &v in &vs {\n            while stk.len() > 0 && self.ls[*stk.last().unwrap()]\
    \ < self.fs[v] {\n                stk.pop();\n            }\n            if let\
    \ Some(&u) = stk.last() {\n                es.push((self.idx[u], self.idx[v]));\n\
    \            }\n            stk.push(v);\n        }\n        (Graph::from_unweighted_directed_edges(vs.len(),\
    \ &es), idx)\n    }\n\n    fn ett_dfs(&mut self, v: usize, p: usize, d: usize,\
    \ g: &Graph<(), ()>) {\n        let c = self.st[0].len();\n        self.fs[v]\
    \ = c;\n        self.st[0].push(v);\n        self.depth[v] = d;\n        for &(u,\
    \ _) in &g[v] {\n            if u == p {\n                continue;\n        \
    \    }\n            self.ett_dfs(u, v, d + 1, g);\n            self.st[0].push(v);\n\
    \        }\n        self.ls[v] = self.st[0].len() - 1;\n    }\n\n    fn lca(&self,\
    \ u: usize, v: usize) -> usize {\n        let mut x = self.fs[u];\n        let\
    \ mut y = self.fs[v];\n        if x > y {\n            std::mem::swap(&mut x,\
    \ &mut y);\n        }\n        let l = self.lg[y - x + 1];\n        if self.depth[self.st[l][x]]\
    \ <= self.depth[self.st[l][y + 1 - (1 << l)]] {\n            self.st[l][x]\n \
    \       } else {\n            self.st[l][y + 1 - (1 << l)]\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/graph/compressed-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-10 09:38:39+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/compressed-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/compressed-tree/src/lib.rs
- /library/crates/graph/compressed-tree/src/lib.rs.html
title: crates/graph/compressed-tree/src/lib.rs
---
