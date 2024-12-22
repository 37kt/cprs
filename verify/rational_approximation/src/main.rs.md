---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/rational/src/lib.rs
    title: crates/algebraic/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/stern-brocot-tree/src/lib.rs
    title: crates/math/stern-brocot-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/rational_approximation
    links:
    - https://judge.yosupo.jp/problem/rational_approximation
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rational_approximation\n\
    \nuse proconio::input;\nuse rational::Rational;\nuse stern_brocot_tree::SternBrocotTreeNode;\n\
    \nfn main() {\n    input! {\n        t: usize,\n    }\n\n    for _ in 0..t {\n\
    \        input! {\n            n: i64,\n            x: i64,\n            y: i64,\n\
    \        }\n        let r = Rational::new(x, y);\n        let (Rational { num:\
    \ a, den: b }, _) = SternBrocotTreeNode::binary_search(|x| x <= r, n);\n     \
    \   let (_, Rational { num: c, den: d }) = SternBrocotTreeNode::binary_search(|x|\
    \ x >= r, n);\n        println!(\"{} {} {} {}\", a, b, c, d);\n    }\n}\n"
  dependsOn:
  - crates/algebraic/rational/src/lib.rs
  - crates/math/stern-brocot-tree/src/lib.rs
  isVerificationFile: true
  path: verify/rational_approximation/src/main.rs
  requiredBy: []
  timestamp: '2024-12-22 00:14:04+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/rational_approximation/src/main.rs
layout: document
redirect_from:
- /verify/verify/rational_approximation/src/main.rs
- /verify/verify/rational_approximation/src/main.rs.html
title: verify/rational_approximation/src/main.rs
---
