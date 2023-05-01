---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/division_of_polynomials
    links:
    - https://judge.yosupo.jp/problem/division_of_polynomials
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/division_of_polynomials\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse formal_power_series::FPS;\nuse\
    \ itertools::Itertools;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        m: usize,\n        f: [Mint; n],\n\
    \        g: [Mint; m],\n    }\n    let f = FPS(f);\n    let g = FPS(g);\n    let\
    \ (q, r) = f.divmod(&g);\n    println!(\"{} {}\", q.len(), r.len());\n    println!(\"\
    {}\", q.iter().join(\" \"));\n    println!(\"{}\", r.iter().join(\" \"));\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/division_of_polynomials/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/division_of_polynomials/src/main.rs
layout: document
redirect_from:
- /verify/verify/division_of_polynomials/src/main.rs
- /verify/verify/division_of_polynomials/src/main.rs.html
title: verify/division_of_polynomials/src/main.rs
---
