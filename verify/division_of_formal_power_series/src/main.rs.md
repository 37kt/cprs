---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/division_of_formal_power_series
    links:
    - https://judge.yosupo.jp/problem/division_of_formal_power_series
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/division_of_formal_power_series\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse formal_power_series::FPS;\nuse\
    \ itertools::Itertools;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        m: usize,\n        f: [Mint; n],\n\
    \        g: [Mint; m],\n    }\n    let f = FPS(f);\n    let g = FPS(g);\n    let\
    \ (q, r) = f.divmod(&g);\n    println!(\"{} {}\", q.len(), r.len());\n    println!(\"\
    {}\", q.iter().join(\" \"));\n    println!(\"{}\", r.iter().join(\" \"));\n}\n"
  dependsOn:
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: true
  path: verify/division_of_formal_power_series/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 13:06:23+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/division_of_formal_power_series/src/main.rs
layout: document
redirect_from:
- /verify/verify/division_of_formal_power_series/src/main.rs
- /verify/verify/division_of_formal_power_series/src/main.rs.html
title: verify/division_of_formal_power_series/src/main.rs
---
