---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/discrete-logarithm/src/lib.rs
    title: crates/math/discrete-logarithm/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/discrete_logarithm_mod
    links:
    - https://judge.yosupo.jp/problem/discrete_logarithm_mod
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/discrete_logarithm_mod\n\
    \nuse discrete_logarithm::discrete_logarithm;\nuse modint::DynamicModInt as Mint;\n\
    use proconio::{fastout, input};\n\n#[fastout]\nfn main() {\n    input! {\n   \
    \     t: usize,\n    }\n    for _ in 0..t {\n        input! {\n            x:\
    \ u32,\n            y: u32,\n            m: u32,\n        }\n        Mint::set_modulus(m);\n\
    \        let x = Mint::new(x);\n        let y = Mint::new(y);\n        if let\
    \ Some(k) =\n            discrete_logarithm(Mint::new(1), y, x, |&f, &x| f * x,\
    \ |&f, &g| f * g, m as _)\n        {\n            println!(\"{}\", k);\n     \
    \   } else {\n            println!(\"-1\");\n        }\n    }\n}\n"
  dependsOn:
  - crates/math/discrete-logarithm/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/discrete_logarithm_mod/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/discrete_logarithm_mod/src/main.rs
layout: document
redirect_from:
- /verify/verify/discrete_logarithm_mod/src/main.rs
- /verify/verify/discrete_logarithm_mod/src/main.rs.html
title: verify/discrete_logarithm_mod/src/main.rs
---
