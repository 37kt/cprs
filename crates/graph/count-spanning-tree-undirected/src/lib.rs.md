---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/matrix/src/lib.rs
    title: crates/math/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/count_spanning_tree_undirected/src/main.rs
    title: verify/count_spanning_tree_undirected/src/main.rs
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
  code: "use matrix::Matrix;\nuse modint::ModInt;\n\n/// g[i][j] = (i, j) \u306E\u672C\
    \u6570\npub fn count_spanning_tree_undirected<M>(g: &[Vec<usize>]) -> M\nwhere\n\
    \    M: ModInt,\n{\n    let n = g.len();\n    assert!(g.iter().all(|v| v.len()\
    \ == n));\n\n    let mut a = Matrix::<M>::from(vec![vec![M::default(); n - 1];\
    \ n - 1]);\n    for i in 0..n {\n        for j in i + 1..n {\n            if i\
    \ < n - 1 && j < n - 1 {\n                a[i][j] -= g[i][j].into();\n       \
    \         a[j][i] -= g[i][j].into();\n            }\n            if i < n - 1\
    \ {\n                a[i][i] += g[i][j].into();\n            }\n            if\
    \ j < n - 1 {\n                a[j][j] += g[i][j].into();\n            }\n   \
    \     }\n    }\n\n    a.det()\n}\n"
  dependsOn:
  - crates/math/matrix/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/graph/count-spanning-tree-undirected/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-12 04:49:04+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/count_spanning_tree_undirected/src/main.rs
documentation_of: crates/graph/count-spanning-tree-undirected/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/count-spanning-tree-undirected/src/lib.rs
- /library/crates/graph/count-spanning-tree-undirected/src/lib.rs.html
title: crates/graph/count-spanning-tree-undirected/src/lib.rs
---
