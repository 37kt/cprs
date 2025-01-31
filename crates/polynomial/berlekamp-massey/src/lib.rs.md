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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use modint::ModInt;\n\n/// \u7DDA\u5F62\u56DE\u5E30\u6570\u5217 a \u306E\u524D\
    \ n \u9805\u304B\u3089\u7DDA\u5F62\u6F38\u5316\u5F0F\u3092\u6C42\u3081\u308B\u3002\
    \  \n/// \u03A3_{i=0}^{\u221E} a\\[i\\] x^i = P(x) / Q(x) \u3068\u8868\u305B\u308B\
    \u3002  \n/// \u3053\u306E\u3068\u304D\u306E Q(x) \u306E\u4FC2\u6570\u3092\u6C42\
    \u3081\u308B\u3002  \n/// deg(Q(x)) \u304C\u6700\u5C0F\u304B\u3064 \\[x^0\\] Q(x)\
    \ = 1 \u3068\u306A\u308B\u3082\u306E\u3092\u8FD4\u3059\u3002  \n/// P(x) \u306F\
    \u3001A(x)Q(x) mod (x^n) \u3067\u6C42\u3081\u3089\u308C\u308B\u3002\n///\n///\
    \ \u8A08\u7B97\u91CF: O(n^2)\npub fn berlekamp_massey<T: ModInt>(a: &[T]) -> Vec<T>\
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
  timestamp: '2024-12-30 09:13:10+00:00'
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
