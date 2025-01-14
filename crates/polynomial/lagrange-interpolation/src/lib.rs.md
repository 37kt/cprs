---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/combination/src/lib.rs
    title: crates/number-theory/combination/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use combination::Combination;\nuse modint::ModInt;\n\npub fn lagrange_interpolation<T:\
    \ ModInt>(ys: &[T], x: usize) -> T {\n    let n = ys.len() - 1;\n    if x <= n\
    \ {\n        return ys[x];\n    }\n    let mut res = 0.into();\n    let mut dp\
    \ = vec![1.into(); n + 1];\n    let mut pd = vec![1.into(); n + 1];\n    for i\
    \ in 0..n {\n        dp[i + 1] = dp[i] * (x - i).into();\n    }\n    for i in\
    \ (1..=n).rev() {\n        pd[i - 1] = pd[i] * (x - i).into();\n    }\n    let\
    \ comb = Combination::<T>::new();\n    for i in 0..=n {\n        let tmp = ys[i]\
    \ * dp[i] * pd[i] * comb.fact_inv(i) * comb.fact_inv(n - i);\n        if (n -\
    \ i) & 1 == 1 {\n            res -= tmp;\n        } else {\n            res +=\
    \ tmp;\n        }\n    }\n    res\n}\n"
  dependsOn:
  - crates/number-theory/combination/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/lagrange-interpolation/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/polynomial/lagrange-interpolation/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/lagrange-interpolation/src/lib.rs
- /library/crates/polynomial/lagrange-interpolation/src/lib.rs.html
title: crates/polynomial/lagrange-interpolation/src/lib.rs
---
