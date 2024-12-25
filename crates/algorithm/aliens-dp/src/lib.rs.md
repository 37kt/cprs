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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// Aliens DP (\u51F8\u95A2\u6570)\n///\n/// \u51F8\u95A2\u6570 `f` \u306E\
    \ `f(x)` \u3092\u6C42\u3081\u308B\u3002\n///\n/// # \u6982\u8981\n/// - `g(p)\
    \ := f(x) + px` \u3068\u306A\u308B\u95A2\u6570 `g` \u3092\u7528\u3044\u3066 `f(x)`\
    \ \u3092\u8A08\u7B97\u3059\u308B\n/// - `p_lb \u2264 p \u2264 p_ub` \u306E\u7BC4\
    \u56F2\u3067\u63A2\u7D22\u3092\u884C\u3046\n/// - \u63A2\u7D22\u7BC4\u56F2\u306F\
    \u554F\u984C\u3054\u3068\u306B\u9069\u5207\u306B\u8A2D\u5B9A\u3059\u308B\u5FC5\
    \u8981\u304C\u3042\u308B\npub fn aliens_dp_convex(x: usize, p_lb: i64, p_ub: i64,\
    \ g: impl Fn(i64) -> i64) -> i64 {\n    let x = x as i64;\n    assert!(p_lb <\
    \ p_ub);\n    let mut l = p_lb - 1;\n    let mut r = p_ub + 1;\n    while l +\
    \ 1 < r {\n        let p = l + (r - l) / 2;\n        let c = g(p + 1) - g(p);\n\
    \        if c <= x {\n            r = p;\n        } else {\n            l = p;\n\
    \        }\n    }\n    g(r) - r * x\n}\n\n/// Aliens DP (\u51F9\u95A2\u6570)\n\
    ///\n/// \u51F9\u95A2\u6570 `f` \u306E `f(x)` \u3092\u6C42\u3081\u308B\u3002\n\
    ///\n/// # \u6982\u8981\n/// - `g(p) := f(x) - px` \u3068\u306A\u308B\u95A2\u6570\
    \ `g` \u3092\u7528\u3044\u3066 `f(x)` \u3092\u8A08\u7B97\u3059\u308B\n/// - `p_lb\
    \ \u2264 p \u2264 p_ub` \u306E\u7BC4\u56F2\u3067\u63A2\u7D22\u3092\u884C\u3046\
    \n/// - \u63A2\u7D22\u7BC4\u56F2\u306F\u554F\u984C\u3054\u3068\u306B\u9069\u5207\
    \u306B\u8A2D\u5B9A\u3059\u308B\u5FC5\u8981\u304C\u3042\u308B\npub fn aliens_dp_concave(x:\
    \ usize, p_lb: i64, p_ub: i64, g: impl Fn(i64) -> i64) -> i64 {\n    let x = x\
    \ as i64;\n    assert!(p_lb < p_ub);\n    let mut l = p_lb - 1;\n    let mut r\
    \ = p_ub + 1;\n    while l + 1 < r {\n        let p = l + (r - l) / 2;\n     \
    \   let c = g(p) - g(p + 1);\n        if c <= x {\n            r = p;\n      \
    \  } else {\n            l = p;\n        }\n    }\n    g(r) + r * x\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/aliens-dp/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/aliens-dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/aliens-dp/src/lib.rs
- /library/crates/algorithm/aliens-dp/src/lib.rs.html
title: crates/algorithm/aliens-dp/src/lib.rs
---
