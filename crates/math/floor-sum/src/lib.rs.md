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
  code: "use std::mem::swap;\n\nfn safe_mod(mut x: i64, m: i64) -> i64 {\n    x %=\
    \ m;\n    if x < 0 {\n        x += m;\n    }\n    x\n}\n\npub fn floor_sum_unsigned(mut\
    \ n: u64, mut m: u64, mut a: u64, mut b: u64) -> u64 {\n    let mut res = 0;\n\
    \    loop {\n        if a >= m {\n            res += n * (n - 1) / 2 * (a / m);\n\
    \            a %= m;\n        }\n        if b >= m {\n            res += n * (b\
    \ / m);\n            b %= m;\n        }\n\n        let y_max = a * n + b;\n  \
    \      if y_max < m {\n            break;\n        }\n        n = y_max / m;\n\
    \        b = y_max % m;\n        swap(&mut m, &mut a);\n    }\n    res\n}\n\n\
    pub fn floor_sum(n: u64, m: u64, mut a: i64, mut b: i64) -> i64 {\n    let n =\
    \ n as i64;\n    let m = m as i64;\n    let mut res = 0;\n    if a < 0 {\n   \
    \     let a2 = safe_mod(a, m);\n        res -= n * (n - 1) / 2 * ((a2 - a) / m);\n\
    \        a = a2;\n    }\n    if b < 0 {\n        let b2 = safe_mod(b, m);\n  \
    \      res -= n * ((b2 - b) / m);\n        b = b2;\n    }\n    res + floor_sum_unsigned(n\
    \ as u64, m as u64, a as u64, b as u64) as i64\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/floor-sum/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/floor-sum/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/floor-sum/src/lib.rs
- /library/crates/math/floor-sum/src/lib.rs.html
title: crates/math/floor-sum/src/lib.rs
---
