---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use matrix::Matrix;\nuse modint::ModInt;\n\n/// r \u3092\u6839\u3068\u3059\
    \u308B\u6709\u5411\u5168\u57DF\u6728\u306E\u500B\u6570\u3092\u6570\u3048\u308B\
    \n/// g[i][j] = (i, j) \u306E\u672C\u6570\npub fn count_spanning_tree_directed<M>(g:\
    \ &[Vec<usize>], r: usize) -> M\nwhere\n    M: ModInt,\n{\n    let n = g.len();\n\
    \    assert!(g.iter().all(|v| v.len() == n));\n\n    let mut a = Matrix::<M>::from(vec![vec![M::default();\
    \ n - 1]; n - 1]);\n\n    let conv = |i: usize| if i < r { i } else { i - 1 };\n\
    \n    for i in 0..n {\n        for j in 0..n {\n            let ii = conv(i);\n\
    \            let jj = conv(j);\n            if i != r && j != r {\n          \
    \      a[jj][ii] -= g[i][j].into();\n            }\n            if j != r {\n\
    \                a[jj][jj] += g[i][j].into();\n            }\n        }\n    }\n\
    \n    a.det()\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/graph/count-spanning-tree-directed/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/graph/count-spanning-tree-directed/src/lib.rs
layout: document
redirect_from:
- /library/crates/graph/count-spanning-tree-directed/src/lib.rs
- /library/crates/graph/count-spanning-tree-directed/src/lib.rs.html
title: crates/graph/count-spanning-tree-directed/src/lib.rs
---
