---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution-ntt-friendly/src/lib.rs
    title: crates/convolution/convolution-ntt-friendly/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/wildcard_pattern_matching/src/main.rs
    title: verify/wildcard_pattern_matching/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use convolution_ntt_friendly::convolution_ntt_friendly;\nuse modint::ModInt998244353\
    \ as Mint;\nuse random::Pcg64Fast;\n\npub fn wildcard_pattern_matching<T>(s: &[T],\
    \ t: &[T], wildcard: T) -> Vec<bool>\nwhere\n    T: Copy + Eq + Into<Mint>,\n\
    {\n    let n = s.len();\n    let m = t.len();\n    assert!(n >= m);\n    let mut\
    \ rng = Pcg64Fast::default();\n    let r = Mint::new(rng.u64());\n    let a1:\
    \ Vec<_> = s\n        .iter()\n        .map(|&x| if x == wildcard { Mint::new(0)\
    \ } else { r + x })\n        .collect();\n    let a2: Vec<_> = a1.iter().map(|&x|\
    \ x * x).collect();\n    let a3: Vec<_> = a1.iter().map(|&x| x * x * x).collect();\n\
    \    let b1: Vec<_> = t\n        .iter()\n        .rev()\n        .map(|&x| if\
    \ x == wildcard { Mint::new(0) } else { r + x })\n        .collect();\n    let\
    \ b2: Vec<_> = b1.iter().map(|&x| x * x).collect();\n    let b3: Vec<_> = b1.iter().map(|&x|\
    \ x * x * x).collect();\n    let c13 = convolution_ntt_friendly(a1, b3);\n   \
    \ let c22 = convolution_ntt_friendly(a2, b2);\n    let c31 = convolution_ntt_friendly(a3,\
    \ b1);\n    (0..=n - m)\n        .map(|i| (c13[i + m - 1] - c22[i + m - 1] * 2\
    \ + c31[i + m - 1]).val() == 0)\n        .collect()\n}\n"
  dependsOn:
  - crates/convolution/convolution-ntt-friendly/src/lib.rs
  - crates/misc/random/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/string/wildcard-pattern-matching/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 07:02:27+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/wildcard_pattern_matching/src/main.rs
documentation_of: crates/string/wildcard-pattern-matching/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/wildcard-pattern-matching/src/lib.rs
- /library/crates/string/wildcard-pattern-matching/src/lib.rs.html
title: crates/string/wildcard-pattern-matching/src/lib.rs
---
