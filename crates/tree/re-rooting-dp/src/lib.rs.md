---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use graph::Graph;\n\npub struct ReRootingDP<S>\nwhere\n    S: Clone,\n{\n\
    \    par: Vec<usize>,\n    dp: Vec<S>,\n    dpl: Vec<S>,\n    dph: Vec<S>,\n}\n\
    \nimpl<S> ReRootingDP<S>\nwhere\n    S: Clone,\n{\n    pub fn build<V, E, Merge,\
    \ AddVertex, AddEdge>(\n        g: &Graph<V, E>,\n        identity: S,\n     \
    \   merge: Merge,\n        add_vertex: AddVertex,\n        add_edge: AddEdge,\n\
    \    ) -> Self\n    where\n        V: Copy,\n        E: Copy,\n        Merge:\
    \ Fn(&S, &S) -> S,\n        AddVertex: Fn(&S, V) -> S,\n        AddEdge: Fn(&S,\
    \ E) -> S,\n    {\n        let n = g.size();\n        let mut ord = vec![];\n\
    \        let mut par = vec![!0; n];\n        let mut st = vec![0];\n        while\
    \ let Some(v) = st.pop() {\n            ord.push(v);\n            for &(u, _)\
    \ in g.out_edges(v) {\n                if u != 0 && par[u] == !0 {\n         \
    \           par[u] = v;\n                    st.push(u);\n                }\n\
    \            }\n        }\n\n        let mut dpl = vec![identity.clone(); n];\n\
    \        let mut dph = vec![identity.clone(); n];\n        let mut dp = vec![identity.clone();\
    \ n];\n        for &v in ord.iter().rev() {\n            let m = g.out_edges(v).len();\n\
    \            let mut xl = vec![identity.clone(); m + 1];\n            let mut\
    \ xr = vec![identity.clone(); m + 1];\n            for i in 0..m {\n         \
    \       let u = g.out_edges(v)[i].0;\n                if u == par[v] {\n     \
    \               xl[i + 1] = xl[i].clone();\n                } else {\n       \
    \             xl[i + 1] = merge(&xl[i], &dph[u]);\n                }\n       \
    \     }\n            for i in (0..m).rev() {\n                let u = g.out_edges(v)[i].0;\n\
    \                if u == par[v] {\n                    xr[i] = xr[i + 1].clone();\n\
    \                } else {\n                    xr[i] = merge(&dph[u], &xr[i +\
    \ 1]);\n                }\n            }\n            for i in 0..m {\n      \
    \          let u = g.out_edges(v)[i].0;\n                if u != par[v] {\n  \
    \                  dph[u] = merge(&xl[i], &xr[i + 1]);\n                }\n  \
    \          }\n            dp[v] = xr[0].clone();\n            dpl[v] = add_vertex(&dp[v],\
    \ g.vertex(v));\n            for &(u, w) in g.out_edges(v) {\n               \
    \ if u == par[v] {\n                    dph[v] = add_edge(&dpl[v], w);\n     \
    \           }\n            }\n        }\n        dp[0] = add_vertex(&dp[0], g.vertex(0));\n\
    \        for &(u, _) in g.out_edges(0) {\n            dph[u] = add_vertex(&dph[u],\
    \ g.vertex(0));\n        }\n        for &v in &ord {\n            for &(u, w)\
    \ in g.out_edges(v) {\n                if u == par[v] {\n                    continue;\n\
    \                }\n                let mut x = add_edge(&dph[u], w);\n      \
    \          for &(vv, _) in g.out_edges(u) {\n                    if vv == v {\n\
    \                        continue;\n                    }\n                  \
    \  dph[vv] = merge(&dph[vv], &x);\n                    dph[vv] = add_vertex(&dph[vv],\
    \ g.vertex(u));\n                }\n                x = merge(&dp[u], &x);\n \
    \               dp[u] = add_vertex(&x, g.vertex(u));\n            }\n        }\n\
    \        Self { dp, par, dpl, dph }\n    }\n\n    pub fn prod(&self, v: usize)\
    \ -> S {\n        self.dp[v].clone()\n    }\n\n    pub fn prod_subtree(&self,\
    \ v: usize, p: usize) -> S {\n        assert_ne!(p, !0);\n        if self.par[v]\
    \ == p {\n            self.dpl[v].clone()\n        } else {\n            self.dph[p].clone()\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  isVerificationFile: false
  path: crates/tree/re-rooting-dp/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-17 15:25:34+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/tree/re-rooting-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/re-rooting-dp/src/lib.rs
- /library/crates/tree/re-rooting-dp/src/lib.rs.html
title: crates/tree/re-rooting-dp/src/lib.rs
---
