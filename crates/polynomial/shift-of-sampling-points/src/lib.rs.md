---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal-power-series/src/lib.rs
    title: crates/polynomial/formal-power-series/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/shift_of_sampling_points_of_polynomial/src/main.rs
    title: verify/shift_of_sampling_points_of_polynomial/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use formal_power_series::fps;\nuse modint::StaticModInt;\n\n/// f(0), f(1),\
    \ ..., f(n-1) \u304B\u3089 f(c+i), f(c+1+i), ..., f(c+m-1) \u3092\u6C42\u3081\u308B\
    \npub fn shift_of_sampling_points<const P: u32>(\n    ys: &[StaticModInt<P>],\n\
    \    c: StaticModInt<P>,\n    m: usize,\n) -> Vec<StaticModInt<P>> {\n    let\
    \ c = c.val() as usize;\n    let n = ys.len();\n    if c < n {\n        let mut\
    \ res = ys[c..].to_vec();\n        res.truncate(m);\n        if n < c + m {\n\
    \            let mut suf = shift_of_sampling_points(ys, n.into(), m - res.len());\n\
    \            res.append(&mut suf);\n        }\n        return res;\n    }\n\n\
    \    if c + m > P as usize {\n        let mut pre = shift_of_sampling_points(ys,\
    \ c.into(), P as usize - c);\n        let mut suf = shift_of_sampling_points(ys,\
    \ 0.into(), m - pre.len());\n        pre.append(&mut suf);\n        return pre;\n\
    \    }\n\n    let mut fact_inv = vec![StaticModInt::new(1); n];\n    let mut d\
    \ = fps![1; n];\n    for i in 2..n {\n        fact_inv[n - 1] *= i;\n    }\n \
    \   fact_inv[n - 1] = fact_inv[n - 1].inv();\n    for i in (1..n).rev() {\n  \
    \      fact_inv[i - 1] = fact_inv[i] * i;\n    }\n    for i in 0..n {\n      \
    \  d[i] = fact_inv[i] * fact_inv[n - 1 - i] * ys[i];\n        if (n - 1 - i) &\
    \ 1 != 0 {\n            d[i] = -d[i];\n        }\n    }\n\n    let mut h = fps![0;\
    \ m + n - 1];\n    for i in 0..m + n - 1 {\n        h[i] = StaticModInt::new(c\
    \ + 1 + i - n).inv();\n    }\n\n    let dh = d * &h;\n\n    let mut res = fps![0;\
    \ m];\n    let mut cur = StaticModInt::new(c);\n    for i in 1..n {\n        cur\
    \ *= c - i;\n    }\n    for i in 0..m {\n        res[i] = cur * dh[n - 1 + i];\n\
    \        cur *= c + i + 1;\n        cur *= h[i];\n    }\n    res.0\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/polynomial/formal-power-series/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/shift-of-sampling-points/src/lib.rs
  requiredBy: []
  timestamp: '2023-09-24 09:50:05+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/shift_of_sampling_points_of_polynomial/src/main.rs
documentation_of: crates/polynomial/shift-of-sampling-points/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/shift-of-sampling-points/src/lib.rs
- /library/crates/polynomial/shift-of-sampling-points/src/lib.rs.html
title: crates/polynomial/shift-of-sampling-points/src/lib.rs
---
