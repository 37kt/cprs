---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':question:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/tree_path_composite_sum/src/main.rs
    title: verify/tree_path_composite_sum/src/main.rs
  - icon: ':x:'
    path: verify/yuki1333/src/main.rs
    title: verify/yuki1333/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::{Act, Monoid};\nuse graph::Graph;\n\npub struct ReRootingDP<S>\n\
    where\n    S: Clone,\n{\n    par: Vec<usize>,\n    dp: Vec<S>,\n    dpl: Vec<S>,\n\
    \    dph: Vec<S>,\n}\n\nimpl<T> ReRootingDP<T>\nwhere\n    T: Clone,\n{\n    pub\
    \ fn build<M, V, E>(g: &Graph<V::S, E::S>) -> Self\n    where\n        M: Monoid<S\
    \ = T>,\n        M::S: Clone,\n        V: Act<X = M::S>,\n        V::S: Clone,\n\
    \        E: Act<X = M::S>,\n        E::S: Clone,\n    {\n        let n = g.size();\n\
    \        let mut ord = vec![];\n        let mut par = vec![!0; n];\n        let\
    \ mut st = vec![0];\n        while let Some(v) = st.pop() {\n            ord.push(v);\n\
    \            for &(u, _) in g.out_edges(v) {\n                if u != 0 && par[u]\
    \ == !0 {\n                    par[u] = v;\n                    st.push(u);\n\
    \                }\n            }\n        }\n\n        let mut dpl = vec![M::e();\
    \ n];\n        let mut dph = vec![M::e(); n];\n        let mut dp = vec![M::e();\
    \ n];\n        for &v in ord.iter().rev() {\n            let es = g.out_edges(v).collect::<Vec<_>>();\n\
    \            let m = es.len();\n            let mut xl = vec![M::e(); m + 1];\n\
    \            let mut xr = vec![M::e(); m + 1];\n            for i in 0..m {\n\
    \                let &(u, _) = es[i];\n                if u == par[v] {\n    \
    \                xl[i + 1] = xl[i].clone();\n                } else {\n      \
    \              xl[i + 1] = M::op(&xl[i], &dph[u]);\n                }\n      \
    \      }\n            for i in (0..m).rev() {\n                let &(u, _) = es[i];\n\
    \                if u == par[v] {\n                    xr[i] = xr[i + 1].clone();\n\
    \                } else {\n                    xr[i] = M::op(&dph[u], &xr[i +\
    \ 1]);\n                }\n            }\n            for i in 0..m {\n      \
    \          let &(u, _) = es[i];\n                if u != par[v] {\n          \
    \          dph[u] = M::op(&xl[i], &xr[i + 1]);\n                }\n          \
    \  }\n            dp[v] = xr[0].clone();\n            dpl[v] = V::act(&g.vertices()[v],\
    \ &dp[v]);\n            for (u, w) in g.out_edges(v) {\n                if *u\
    \ == par[v] {\n                    dph[v] = E::act(&w, &dpl[v]);\n           \
    \     }\n            }\n        }\n        dp[0] = V::act(&g.vertices()[0], &dp[0]);\n\
    \        for &(u, _) in g.out_edges(0) {\n            dph[u] = V::act(&g.vertices()[0],\
    \ &dph[u]);\n        }\n        for &v in &ord {\n            for (u, w) in g.out_edges(v)\
    \ {\n                if *u == par[v] {\n                    continue;\n      \
    \          }\n                let mut x = E::act(&w, &dph[*u]);\n            \
    \    for &(vv, _) in g.out_edges(*u) {\n                    if vv == v {\n   \
    \                     continue;\n                    }\n                    dph[vv]\
    \ = M::op(&dph[vv], &x);\n                    dph[vv] = V::act(&g.vertices()[*u],\
    \ &dph[vv]);\n                }\n                x = M::op(&dp[*u], &x);\n   \
    \             dp[*u] = V::act(&g.vertices()[*u], &x);\n            }\n       \
    \ }\n        Self { par, dp, dpl, dph }\n    }\n\n    pub fn prod(&self, v: usize)\
    \ -> T {\n        self.dp[v].clone()\n    }\n\n    pub fn prod_subtree(&self,\
    \ v: usize, p: usize) -> T {\n        if p == !0 {\n            self.dp[v].clone()\n\
    \        } else if self.par[v] == p {\n            self.dpl[v].clone()\n     \
    \   } else {\n            self.dph[p].clone()\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/re-rooting-dp/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-17 16:30:46+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/yuki1333/src/main.rs
  - verify/tree_path_composite_sum/src/main.rs
documentation_of: crates/tree/re-rooting-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/re-rooting-dp/src/lib.rs
- /library/crates/tree/re-rooting-dp/src/lib.rs.html
title: crates/tree/re-rooting-dp/src/lib.rs
---
