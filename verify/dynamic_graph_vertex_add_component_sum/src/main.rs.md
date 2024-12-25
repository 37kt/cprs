---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
    title: crates/algorithm/offline-dynamic-connectivity/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/rollback-union-find-component-sum/src/lib.rs
    title: crates/data-structure/rollback-union-find-component-sum/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/dynamic_graph_vertex_add_component_sum
    links:
    - https://judge.yosupo.jp/problem/dynamic_graph_vertex_add_component_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_graph_vertex_add_component_sum\n\
    \nuse algebraic::{algebra, monoid};\nuse offline_dynamic_connectivity::{OfflineDynamicConnectivity,\
    \ RollbackUnionFindTrait};\nuse proconio::input;\nuse rollback_union_find_component_sum::RollbackUnionFindComponentSum;\n\
    \nalgebra!(M, usize);\nmonoid!(M, 0, |a, b| a + b);\n\nstruct UF {\n    uf: RollbackUnionFindComponentSum<M>,\n\
    }\n\nimpl RollbackUnionFindTrait for UF {\n    type Query = usize;\n\n    fn add_edge(&mut\
    \ self, x: usize, y: usize) {\n        self.uf.merge(x, y);\n    }\n\n    fn undo(&mut\
    \ self) {\n        self.uf.undo();\n    }\n\n    fn get(&mut self, query: Self::Query)\
    \ {\n        println!(\"{}\", self.uf.sum(query));\n    }\n}\n\nfn main() {\n\
    \    input! {\n        n: usize,\n        q: usize,\n        mut a: [usize; n],\n\
    \    }\n    a.resize(n + q, 0);\n    let mut qs = vec![];\n    for i in 0..q {\n\
    \        input! {\n            t: usize,\n        }\n        if t != 3 {\n   \
    \         input! {\n                u: usize,\n                v: usize,\n   \
    \         }\n            qs.push((t, u, v));\n            if t == 2 {\n      \
    \          a[n + i] = v;\n            }\n        } else {\n            input!\
    \ {\n                v: usize,\n            }\n            qs.push((t, v, 0));\n\
    \        }\n    }\n    let mut dc = OfflineDynamicConnectivity::new(UF {\n   \
    \     uf: RollbackUnionFindComponentSum::new(&a),\n    });\n    for i in 0..q\
    \ {\n        let (t, u, v) = qs[i];\n        match t {\n            0 => dc.add_edge(u,\
    \ v),\n            1 => dc.remove_edge(u, v),\n            2 => dc.add_edge(u,\
    \ n + i),\n            3 => dc.get(u),\n            _ => unreachable!(),\n   \
    \     }\n    }\n    dc.run();\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/algorithm/offline-dynamic-connectivity/src/lib.rs
  - crates/data-structure/rollback-union-find-component-sum/src/lib.rs
  isVerificationFile: true
  path: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/dynamic_graph_vertex_add_component_sum/src/main.rs
- /verify/verify/dynamic_graph_vertex_add_component_sum/src/main.rs.html
title: verify/dynamic_graph_vertex_add_component_sum/src/main.rs
---
