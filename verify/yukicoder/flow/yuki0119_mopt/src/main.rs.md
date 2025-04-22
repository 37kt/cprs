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
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/119
    links:
    - https://yukicoder.me/problems/no/119
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/119\n\n\
    use multivalued_optimization::MultivaluedOptimization;\nuse proconio::{fastout,\
    \ input};\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n      \
    \  bc: [(i64, i64); n],\n        m: usize,\n        de: [(usize, usize); m],\n\
    \    }\n\n    let mut opt = MultivaluedOptimization::new(vec![3; n]);\n    for\
    \ (i, &(b, c)) in bc.iter().enumerate() {\n        opt.add_unary(i, |mi| Some([-c,\
    \ 0, -b][mi]));\n    }\n    for &(i, j) in &de {\n        opt.add_binary(\n  \
    \          i,\n            j,\n            |mi, mj| {\n                if mi ==\
    \ 2 && mj == 0 {\n                    None\n                } else {\n       \
    \             Some(0)\n                }\n            },\n        );\n    }\n\n\
    \    let (cost, choice) = opt.solve();\n\n    let mut sum = 0;\n    for i in 0..n\
    \ {\n        let (b, c) = bc[i];\n        sum += [-c, 0, -b][choice[i]];\n   \
    \ }\n    assert_eq!(cost, sum);\n    for &(i, j) in &de {\n        assert!(!(choice[i]\
    \ == 2 && choice[j] == 0));\n    }\n\n    println!(\"{}\", -cost);\n}\n"
  dependsOn:
  - crates/flow/multivalued_optimization/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/flow/yuki0119_mopt/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/flow/yuki0119_mopt/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/flow/yuki0119_mopt/src/main.rs
- /verify/verify/yukicoder/flow/yuki0119_mopt/src/main.rs.html
title: verify/yukicoder/flow/yuki0119_mopt/src/main.rs
---
