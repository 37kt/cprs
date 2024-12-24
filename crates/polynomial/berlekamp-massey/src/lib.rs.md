---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/find_linear_recurrence/src/main.rs
    title: verify/find_linear_recurrence/src/main.rs
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
  code: "use modint::ModInt;\n\npub fn berlekamp_massey<T: ModInt>(a: &[T]) -> Vec<T>\
    \ {\n    let n = a.len();\n    let mut b = Vec::with_capacity(n + 1);\n    let\
    \ mut c = Vec::with_capacity(n + 1);\n    b.push(1.into());\n    c.push(1.into());\n\
    \    let mut y = 1.into();\n    for ed in 1..=n {\n        let l = c.len();\n\
    \        let mut m = b.len();\n        let mut x: T = 0.into();\n        for i\
    \ in 0..l {\n            x += c[i] * a[ed - l + i];\n        }\n        b.push(0.into());\n\
    \        m += 1;\n        if x.val() == 0 {\n            continue;\n        }\n\
    \        let freq = x / y;\n        if l < m {\n            let tmp = c.clone();\n\
    \            c.resize(m, 0.into());\n            c.rotate_right(m - l);\n    \
    \        for i in 0..m {\n                c[m - 1 - i] -= freq * b[m - 1 - i];\n\
    \            }\n            b = tmp;\n            y = x;\n        } else {\n \
    \           for i in 0..m {\n                c[l - 1 - i] -= freq * b[m - 1 -\
    \ i];\n            }\n        }\n    }\n    c.reverse();\n    c\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: false
  path: crates/polynomial/berlekamp-massey/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-24 03:04:37+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/find_linear_recurrence/src/main.rs
documentation_of: crates/polynomial/berlekamp-massey/src/lib.rs
layout: document
redirect_from:
- /library/crates/polynomial/berlekamp-massey/src/lib.rs
- /library/crates/polynomial/berlekamp-massey/src/lib.rs.html
title: crates/polynomial/berlekamp-massey/src/lib.rs
---
