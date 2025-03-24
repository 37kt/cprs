---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/chminmax.rs
    title: crates/misc/macros/src/chminmax.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint_61::ModInt61;\nuse proconio::fastout;\n\n#[fastout]\nfn main()\
    \ {\n    let timer = std::time::Instant::now();\n    let mut rng = random::Pcg64Fast::default();\n\
    \    const N: usize = 1_000_000_000;\n    let x = ModInt61::new(rng.u64());\n\
    \    let mut s = ModInt61::new(1);\n    for _ in 0..N {\n        s *= x;\n   \
    \ }\n    println!(\"{}\", s);\n    println!(\"{} ms\", timer.elapsed().as_millis());\n\
    }\n"
  dependsOn:
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/mvec.rs
  - crates/misc/macros/src/yes.rs
  - crates/misc/random/src/lib.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  isVerificationFile: false
  path: verify/sandbox/test/src/main.rs
  requiredBy: []
  timestamp: '2025-03-24 01:42:22+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/test/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/test/src/main.rs
- /library/verify/sandbox/test/src/main.rs.html
title: verify/sandbox/test/src/main.rs
---
