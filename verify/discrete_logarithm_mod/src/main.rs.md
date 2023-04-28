---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/discrete-logarithm/src/lib.rs
    title: crates/math/discrete-logarithm/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/discrete_logarithm_mod
    links:
    - https://judge.yosupo.jp/problem/discrete_logarithm_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod\n\
    \nuse ac_library::ModInt;\nuse algebraic::{act, algebra, monoid};\nuse discrete_logarithm::discrete_logarithm;\n\
    use proconio::input;\n\nalgebra!(M, ModInt);\nmonoid!(M, ModInt::new(0), |f, g|\
    \ f * g);\nact!(M, ModInt, |f, x| f * x);\n\n#[proconio::fastout]\nfn main() {\n\
    \    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n        input! {\n\
    \            x: u32,\n            y: u32,\n            m: u32,\n        }\n  \
    \      ModInt::set_modulus(m);\n        let x = ModInt::new(x);\n        let y\
    \ = ModInt::new(y);\n        if let Some(k) = discrete_logarithm::<M>(ModInt::new(1),\
    \ y, x, m as usize) {\n            println!(\"{}\", k);\n        } else {\n  \
    \          println!(\"-1\");\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/math/discrete-logarithm/src/lib.rs
  isVerificationFile: true
  path: verify/discrete_logarithm_mod/src/main.rs
  requiredBy: []
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/discrete_logarithm_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/discrete_logarithm_mod/src/main.rs
- /verify/verify/discrete_logarithm_mod/src/main.rs.html
title: verify/discrete_logarithm_mod/src/main.rs
---