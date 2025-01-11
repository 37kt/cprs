---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/discrete_logarithm_mod/src/main.rs
    title: verify/discrete_logarithm_mod/src/main.rs
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
  code: "use std::collections::HashSet;\n\n/// f^n s = t \u3068\u306A\u308B\u6700\u521D\
    \u306E n (n < lim) \u304C\u5B58\u5728\u3059\u308B\u306A\u3089\u8FD4\u3059\npub\
    \ fn discrete_logarithm<X, F>(\n    mut s: X,\n    t: X,\n    f: F,\n    mut act:\
    \ impl FnMut(&F, &X) -> X,\n    mut composition: impl FnMut(&F, &F) -> F,\n  \
    \  lim: usize,\n) -> Option<usize>\nwhere\n    X: Clone + std::hash::Hash + Eq,\n\
    \    F: Clone,\n{\n    let mut fpow = |mut n: usize| {\n        assert!(n > 0);\n\
    \        n -= 1;\n        let mut p = f.clone();\n        let mut res = f.clone();\n\
    \        while n > 0 {\n            if n & 1 == 1 {\n                res = composition(&res,\
    \ &p);\n            }\n            p = composition(&p, &p);\n            n >>=\
    \ 1;\n        }\n        res\n    };\n\n    let m = (lim as f64).sqrt().ceil()\
    \ as usize;\n    let mut st = HashSet::new();\n    let mut tt = t.clone();\n \
    \   for _ in 0..m {\n        tt = act(&f, &tt);\n        st.insert(tt.clone());\n\
    \    }\n    let g = fpow(m);\n    let mut failed = false;\n    for i in 0..=m\
    \ {\n        let s1 = act(&g, &s);\n        if st.contains(&s1) {\n          \
    \  for j in 0..m {\n                if s == t {\n                    let res =\
    \ i * m + j;\n                    return if res >= lim { None } else { Some(res)\
    \ };\n                }\n                s = act(&f, &s);\n            }\n   \
    \         if failed {\n                return None;\n            }\n         \
    \   failed = true;\n        }\n        s = s1;\n    }\n\n    None\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/math/discrete-logarithm/src/lib.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/discrete_logarithm_mod/src/main.rs
documentation_of: crates/math/discrete-logarithm/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/discrete-logarithm/src/lib.rs
- /library/crates/math/discrete-logarithm/src/lib.rs.html
title: crates/math/discrete-logarithm/src/lib.rs
---
