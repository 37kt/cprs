---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/arbitrary_mod.rs
    title: crates/convolution/convolution/src/arbitrary_mod.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/mod_2_64.rs
    title: crates/convolution/convolution/src/mod_2_64.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/naive.rs
    title: crates/convolution/convolution/src/naive.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt.rs
    title: crates/convolution/convolution/src/ntt.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/ntt_friendly.rs
    title: crates/convolution/convolution/src/ntt_friendly.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
    title: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
    title: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/numeric.rs
    title: crates/number_theory/modint/static_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ops.rs
    title: crates/number_theory/modint/static_modint/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/convolution_mod_1000000007
    links:
    - https://judge.yosupo.jp/problem/convolution_mod_1000000007
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/convolution_mod_1000000007\n\
    \nuse convolution::convolution_arbitrary_mod;\nuse proconio::fastout;\nuse proconio::input;\n\
    use static_modint::ModInt1000000007 as Mint;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        m: usize,\n        a: [Mint; n],\n        b: [Mint;\
    \ m],\n    }\n    let c = convolution_arbitrary_mod(&a, &b);\n    for i in 0..c.len()\
    \ {\n        print!(\"{}\", c[i]);\n        print!(\"{}\", if i == c.len() - 1\
    \ { \"\\n\" } else { \" \" });\n    }\n}\n"
  dependsOn:
  - crates/convolution/convolution/src/arbitrary_mod.rs
  - crates/convolution/convolution/src/lib.rs
  - crates/convolution/convolution/src/mod_2_64.rs
  - crates/convolution/convolution/src/naive.rs
  - crates/convolution/convolution/src/ntt.rs
  - crates/convolution/convolution/src/ntt_friendly.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  isVerificationFile: true
  path: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
  requiredBy: []
  timestamp: '2025-05-17 08:18:49+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
- /verify/verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs.html
title: verify/library_checker/convolution/convolution_mod_1000000007/src/main.rs
---
