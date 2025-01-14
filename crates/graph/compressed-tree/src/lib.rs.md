---
data:
  _extendedDependsOn:
  - icon: ':question:'
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
  code: "use graph::UndirectedGraph;\n\n// Reference:\n// https://tjkendev.github.io/procon-library/cpp/graph/auxiliary_tree.html\n\
    \n/// \u6307\u5B9A\u3055\u308C\u305F\u9802\u70B9\u305F\u3061\u306E\u6700\u5C0F\
    \u5171\u901A\u7956\u5148\u95A2\u4FC2\u3092\u4FDD\u3063\u3066\u6728\u3092\u5727\
    \u7E2E\u3057\u3066\u3067\u304D\u308B\u88DC\u52A9\u7684\u306A\u6728\npub struct\
    \ CompressedTree {\n    fs: Vec<usize>,\n    ls: Vec<usize>,\n    depth: Vec<usize>,\n\
    \    st: Vec<Vec<usize>>,\n    lg: Vec<usize>,\n    idx: Vec<usize>,\n}\n\nimpl\
    \ CompressedTree {\n    /// \u6728 g \u304B\u3089\u3001\u5727\u7E2E\u6728\u3092\
    \u69CB\u7BC9\u3059\u308B\u524D\u6E96\u5099\u3092\u3059\u308B\u3002\n    ///\n\
    \    /// # \u5165\u529B\n    ///\n    /// - `g`: \u6728\n    ///   - \u305F\u3060\
    \u3057\u3001\u9802\u70B9 0 \u3092\u6839\u3068\u3059\u308B\n    ///\n    /// #\
    \ \u8A08\u7B97\u91CF\n    ///\n    /// O(N log N)\n    pub fn new(g: &UndirectedGraph<(),\
    \ ()>) -> Self {\n        let n = g.len();\n        let mut c = Self {\n     \
    \       fs: vec![0; n],\n            ls: vec![0; n],\n            depth: vec![0;\
    \ n],\n            st: vec![vec![]],\n            lg: vec![],\n            idx:\
    \ vec![0; n],\n        };\n\n        c.ett_dfs(0, !0, 0, g);\n\n        c.lg =\
    \ vec![0; 2];\n        let m = c.st[0].len();\n        for i in 2..=m {\n    \
    \        c.lg.push(c.lg[i >> 1] + 1);\n        }\n\n        for i in 0..c.lg[m]\
    \ {\n            c.st.push(vec![]);\n            let b = 1 << i;\n           \
    \ for j in 0..=m - (b << 1) {\n                let t = if c.depth[c.st[i][j]]\
    \ <= c.depth[c.st[i][j + b]] {\n                    c.st[i][j]\n             \
    \   } else {\n                    c.st[i][j + b]\n                };\n       \
    \         c.st[i + 1].push(t);\n            }\n        }\n\n        c\n    }\n\
    \n    /// \u5727\u7E2E\u6728\u3092\u69CB\u7BC9\u3059\u308B\u3002\n    ///\n  \
    \  /// # \u5165\u529B\n    ///\n    /// - `vs`: g \u306E\u9802\u70B9\u306E\u90E8\
    \u5206\u96C6\u5408\n    ///\n    /// # \u51FA\u529B\n    ///\n    /// - `h`: vs\
    \ \u306B\u542B\u307E\u308C\u308B\u9802\u70B9\u540C\u58EB\u306E\u6700\u5C0F\u5171\
    \u901A\u7956\u5148\u95A2\u4FC2\u3092\u5931\u308F\u306A\u3044\u3088\u3046\u306B\
    \u5727\u7E2E\u3057\u305F\u6728\n    /// - `idx`: h \u306E\u9802\u70B9\u306B\u5BFE\
    \u5FDC\u3059\u308B g \u306E\u9802\u70B9\u306E index\n    ///\n    /// # \u8A08\
    \u7B97\u91CF\n    ///\n    /// O(|vs|)\n    pub fn build(&mut self, vs: &[usize])\
    \ -> (UndirectedGraph<(), ()>, Vec<usize>) {\n        let mut vs = vs.to_vec();\n\
    \        vs.sort_by_key(|&v| self.fs[v]);\n        for i in 0..vs.len() - 1 {\n\
    \            vs.push(self.lca(vs[i], vs[i + 1]));\n        }\n        vs.sort_by_key(|&v|\
    \ self.fs[v]);\n        vs.dedup();\n        let mut stk = vec![];\n        let\
    \ mut es = vec![];\n        let mut idx = vec![0; vs.len()];\n        for i in\
    \ 0..vs.len() {\n            self.idx[vs[i]] = i;\n            idx[i] = vs[i];\n\
    \        }\n        for &v in &vs {\n            while stk.len() > 0 && self.ls[*stk.last().unwrap()]\
    \ < self.fs[v] {\n                stk.pop();\n            }\n            if let\
    \ Some(&u) = stk.last() {\n                es.push((self.idx[u], self.idx[v]));\n\
    \            }\n            stk.push(v);\n        }\n        (UndirectedGraph::from_unweighted_edges(vs.len(),\
    \ &es), idx)\n    }\n\n    fn ett_dfs(&mut self, v: usize, p: usize, d: usize,\
    \ g: &UndirectedGraph<(), ()>) {\n        let c = self.st[0].len();\n        self.fs[v]\
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
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/compressed-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/compressed-tree/src/lib.rs
- /library/crates/graph/compressed-tree/src/lib.rs.html
title: crates/graph/compressed-tree/src/lib.rs
---
