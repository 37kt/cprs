---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
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
  code: "use algebraic::{Act, Monoid};\nuse graph::UndirectedGraph;\n\n/// \u5168\u65B9\
    \u4F4D\u6728 dp\npub struct ReRootingDP<S>\nwhere\n    S: Clone,\n{\n    par:\
    \ Vec<usize>,\n    dp: Vec<S>,\n    dpl: Vec<S>,\n    dph: Vec<S>,\n}\n\nimpl<T>\
    \ ReRootingDP<T>\nwhere\n    T: Clone,\n{\n    /// \u5168\u65B9\u4F4D\u6728 dp\
    \ \u3092\u5B9F\u884C\u3059\u308B\n    pub fn build<M, V, E>(g: &UndirectedGraph<V::S,\
    \ E::S>) -> Self\n    where\n        M: Monoid<S = T>,\n        M::S: Clone,\n\
    \        V: Act<X = M::S>,\n        V::S: Clone,\n        E: Act<X = M::S>,\n\
    \        E::S: Clone,\n    {\n        let n = g.len();\n        let mut ord =\
    \ vec![];\n        let mut par = vec![!0; n];\n        let mut st = vec![0];\n\
    \        while let Some(v) = st.pop() {\n            ord.push(v);\n          \
    \  for &(u, _) in &g[v] {\n                if u != 0 && par[u] == !0 {\n     \
    \               par[u] = v;\n                    st.push(u);\n               \
    \ }\n            }\n        }\n\n        let mut dpl = vec![M::e(); n];\n    \
    \    let mut dph = vec![M::e(); n];\n        let mut dp = vec![M::e(); n];\n \
    \       for &v in ord.iter().rev() {\n            let m = g[v].len();\n      \
    \      let mut xl = vec![M::e(); m + 1];\n            let mut xr = vec![M::e();\
    \ m + 1];\n            for i in 0..m {\n                let (u, _) = g[v][i];\n\
    \                if u == par[v] {\n                    xl[i + 1] = xl[i].clone();\n\
    \                } else {\n                    xl[i + 1] = M::op(&xl[i], &dph[u]);\n\
    \                }\n            }\n            for i in (0..m).rev() {\n     \
    \           let (u, _) = g[v][i];\n                if u == par[v] {\n        \
    \            xr[i] = xr[i + 1].clone();\n                } else {\n          \
    \          xr[i] = M::op(&dph[u], &xr[i + 1]);\n                }\n          \
    \  }\n            for i in 0..m {\n                let (u, _) = g[v][i];\n   \
    \             if u != par[v] {\n                    dph[u] = M::op(&xl[i], &xr[i\
    \ + 1]);\n                }\n            }\n            dp[v] = xr[0].clone();\n\
    \            dpl[v] = V::act(&g.vertex(v), &dp[v]);\n            for (u, w) in\
    \ &g[v] {\n                if *u == par[v] {\n                    dph[v] = E::act(&w,\
    \ &dpl[v]);\n                }\n            }\n        }\n        dp[0] = V::act(&g.vertex(0),\
    \ &dp[0]);\n        for &(u, _) in &g[0] {\n            dph[u] = V::act(&g.vertex(0),\
    \ &dph[u]);\n        }\n        for &v in &ord {\n            for (u, w) in &g[v]\
    \ {\n                if *u == par[v] {\n                    continue;\n      \
    \          }\n                let mut x = E::act(&w, &dph[*u]);\n            \
    \    for &(vv, _) in &g[*u] {\n                    if vv == v {\n            \
    \            continue;\n                    }\n                    dph[vv] = M::op(&dph[vv],\
    \ &x);\n                    dph[vv] = V::act(&g.vertex(*u), &dph[vv]);\n     \
    \           }\n                x = M::op(&dp[*u], &x);\n                dp[*u]\
    \ = V::act(&g.vertex(*u), &x);\n            }\n        }\n        Self { par,\
    \ dp, dpl, dph }\n    }\n\n    /// `v` \u3092\u6839\u3068\u3057\u305F\u3068\u304D\
    \u306E `dp[v]` \u306E\u5024\u3092\u8FD4\u3059\n    pub fn prod(&self, v: usize)\
    \ -> T {\n        self.dp[v].clone()\n    }\n\n    /// `p` \u3092\u6839\u3068\u3057\
    \u305F\u3068\u304D\u306E `dp[v]` \u306E\u5024\u3092\u8FD4\u3059  \n    /// `p`\
    \ = `!0` \u306E\u3068\u304D\u306F `v` \u3092\u6839\u3068\u3057\u305F\u3068\u304D\
    \u306E `dp[v]` \u306E\u5024\u3092\u8FD4\u3059  \n    /// `v` \u3068 `p` \u306F\
    \u76F4\u63A5\u8FBA\u3067\u7D50\u3070\u308C\u3066\u3044\u308B\u5FC5\u8981\u304C\
    \u3042\u308B\n    pub fn prod_subtree(&self, v: usize, p: usize) -> T {\n    \
    \    if p == !0 {\n            self.dp[v].clone()\n        } else if self.par[v]\
    \ == p {\n            self.dpl[v].clone()\n        } else if self.par[p] == v\
    \ {\n            self.dph[p].clone()\n        } else {\n            panic!(\"\
    v and p are not connected by an edge\");\n        }\n    }\n\n    /// 0 \u3092\
    \u6839\u3068\u3057\u305F\u3068\u304D\u306E `v` \u306E\u89AA\u3092\u8FD4\u3059\
    \  \n    /// `v` \u304C\u6839\u306E\u3068\u304D\u306F `!0` \u3092\u8FD4\u3059\n\
    \    pub fn par(&self, v: usize) -> usize {\n        self.par[v]\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/re-rooting-dp/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-14 05:25:42+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/tree_path_composite_sum/src/main.rs
documentation_of: crates/tree/re-rooting-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/re-rooting-dp/src/lib.rs
- /library/crates/tree/re-rooting-dp/src/lib.rs.html
title: crates/tree/re-rooting-dp/src/lib.rs
---
