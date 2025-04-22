---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/multivalued_optimization/src/lib.rs
    title: crates/flow/multivalued_optimization/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use multivalued_optimization::MultivaluedOptimization;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  a: [[usize; n]; n],\n    }\n\n    let mut opt = MultivaluedOptimization::new(vec![5;\
    \ n * n]);\n    let id = |i: usize, j: usize| i * n + j;\n    let f = |x: usize,\
    \ y: usize| Some(x.abs_diff(y).pow(2) as i64);\n    #[allow(clippy::needless_range_loop)]\n\
    \    for i in 0..n {\n        for j in 0..n {\n            if a[i][j] != 0 {\n\
    \                opt.add_unary(id(i, j), |x| (x == a[i][j] - 1).then_some(0));\n\
    \            }\n            if i + 1 < n {\n                opt.add_binary(id(i,\
    \ j), id(i + 1, j), f);\n            }\n            if j + 1 < n {\n         \
    \       opt.add_binary(id(i, j), id(i, j + 1), f);\n            }\n        }\n\
    \    }\n\n    let (_, res) = opt.solve();\n    for i in 0..n {\n        for j\
    \ in 0..n {\n            print!(\"{} \", res[id(i, j)] + 1);\n        }\n    \
    \    println!();\n    }\n}\n"
  dependsOn:
  - crates/flow/multivalued_optimization/src/lib.rs
  isVerificationFile: false
  path: verify/sandbox/abc347g/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/abc347g/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/abc347g/src/main.rs
- /library/verify/sandbox/abc347g/src/main.rs.html
title: verify/sandbox/abc347g/src/main.rs
---
