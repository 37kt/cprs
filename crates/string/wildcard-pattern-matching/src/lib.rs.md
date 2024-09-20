---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
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
  dependsOn: []
  isVerificationFile: false
  path: crates/string/wildcard-pattern-matching/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/string/wildcard-pattern-matching/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/wildcard-pattern-matching/src/lib.rs
- /library/crates/string/wildcard-pattern-matching/src/lib.rs.html
title: crates/string/wildcard-pattern-matching/src/lib.rs
---
