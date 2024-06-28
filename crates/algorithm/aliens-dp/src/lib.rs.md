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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn aliens_dp_convex(x: usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) ->\
    \ i64) -> i64 {\n    let x = x as i64;\n    assert!(p_lb < p_ub);\n    let mut\
    \ l = p_lb - 1;\n    let mut r = p_ub + 1;\n    while l + 1 < r {\n        let\
    \ p = l + (r - l) / 2;\n        let c = g(p + 1) - g(p);\n        if c <= x {\n\
    \            r = p;\n        } else {\n            l = p;\n        }\n    }\n\
    \    g(r) - r * x\n}\n\npub fn aliens_dp_concave(x: usize, p_lb: i64, p_ub: i64,\
    \ g: impl Fn(i64) -> i64) -> i64 {\n    let x = x as i64;\n    assert!(p_lb <\
    \ p_ub);\n    let mut l = p_lb - 1;\n    let mut r = p_ub + 1;\n    while l +\
    \ 1 < r {\n        let p = l + (r - l) / 2;\n        let c = g(p) - g(p + 1);\n\
    \        if c <= x {\n            r = p;\n        } else {\n            l = p;\n\
    \        }\n    }\n    g(r) + r * x\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/aliens-dp/src/lib.rs
  requiredBy: []
  timestamp: '2024-04-19 10:41:29+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/aliens-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/aliens-dp/src/lib.rs
- /library/crates/algorithm/aliens-dp/src/lib.rs.html
title: crates/algorithm/aliens-dp/src/lib.rs
---
