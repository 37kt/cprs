---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/flow/binary_optimization/src/lib.rs
    title: crates/flow/binary_optimization/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/2713
    links:
    - https://yukicoder.me/problems/no/2713
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/2713\n\n\
    use binary_optimization::BinaryOptimization;\nuse proconio::{fastout, input, marker::Usize1};\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        m: usize,\n\
    \        a: [i64; n],\n        b: [i64; m],\n        c: [[Usize1]; m],\n    }\n\
    \n    let mut opt = BinaryOptimization::new(n);\n    for (i, &x) in a.iter().enumerate()\
    \ {\n        opt.add_unary(i, |bi| Some([0, x][bi]));\n    }\n    for i in 0..m\
    \ {\n        opt.add_if_all_1(&c[i], -b[i]);\n    }\n\n    let (cost, choice)\
    \ = opt.solve();\n    let mut sum = 0;\n    for i in 0..n {\n        if choice[i]\
    \ == 1 {\n            sum += a[i];\n        }\n    }\n    for i in 0..m {\n  \
    \      if c[i].iter().all(|&j| choice[j] == 1) {\n            sum -= b[i];\n \
    \       }\n    }\n    assert_eq!(cost, sum);\n\n    println!(\"{}\", -cost);\n\
    }\n"
  dependsOn:
  - crates/flow/binary_optimization/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/flow/yuki2713_binopt/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/flow/yuki2713_binopt/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/flow/yuki2713_binopt/src/main.rs
- /verify/verify/yukicoder/flow/yuki2713_binopt/src/main.rs.html
title: verify/yukicoder/flow/yuki2713_binopt/src/main.rs
---
