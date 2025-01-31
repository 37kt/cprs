---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/min-plus-convolution/src/lib.rs
    title: crates/algorithm/min-plus-convolution/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yuki952/src/main.rs
    title: verify/yuki952/src/main.rs
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
  code: "fn dfs(\n    x1: usize,\n    x2: usize,\n    y1: usize,\n    y2: usize,\n\
    \    select: &impl Fn(usize, usize, usize) -> bool,\n    res: &mut [usize],\n\
    ) {\n    if x1 == x2 {\n        return;\n    }\n    let x = (x1 + x2) / 2;\n \
    \   let mut best_y = y1;\n    for y in y1 + 1..y2 {\n        if select(x, best_y,\
    \ y) {\n            best_y = y;\n        }\n    }\n    res[x] = best_y;\n    dfs(x1,\
    \ x, y1, best_y + 1, select, res);\n    dfs(x + 1, x2, best_y, y2, select, res);\n\
    }\n\n/// monotone\u306A\u884C\u5217\u306B\u5BFE\u3059\u308B\u6700\u5C0F\u5024\u306E\
    \u5217\u3092\u6C42\u3081\u308B\n///\n/// # \u6982\u8981\n/// \u884C\u5217 A \u304C\
    \ monotone \u3067\u3042\u308B\u3068\u304D\u3001\u5404\u884C\u306B\u3064\u3044\u3066\
    \u6700\u5C0F\u5024\u3092\u53D6\u308B\u5217\u3092\u52B9\u7387\u7684\u306B\u8A08\
    \u7B97\u3059\u308B\n///\n/// # \u5F15\u6570\n/// - `h`: \u884C\u5217\u306E\u884C\
    \u6570\n/// - `w`: \u884C\u5217\u306E\u5217\u6570\n/// - `select`: \u6BD4\u8F03\
    \u95A2\u6570\u3002\u4EE5\u4E0B\u306E\u6761\u4EF6\u3092\u6E80\u305F\u3059\u5FC5\
    \u8981\u304C\u3042\u308B\n///   - `select(i, j, k)` := `A[i][j] \u2265 A[i][k]`\n\
    ///\n/// # \u623B\u308A\u5024\n/// - \u9577\u3055 `h` \u306E\u30D9\u30AF\u30C8\
    \u30EB\n/// - `res[i]` \u306F `i` \u884C\u76EE\u3067\u6700\u5C0F\u5024\u3092\u53D6\
    \u308B\u5217\u306E\u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\n///\n/// # \u8A08\u7B97\
    \u91CF\n/// - O(TODO)\n///   - h: \u884C\u6570\n///   - w: \u5217\u6570\npub fn\
    \ monotone_minima(\n    h: usize,\n    w: usize,\n    select: impl Fn(usize, usize,\
    \ usize) -> bool,\n) -> Vec<usize> {\n    let mut res = vec![0; h];\n    dfs(0,\
    \ h, 0, w, &select, &mut res);\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/monotone-minima/src/lib.rs
  requiredBy:
  - crates/algorithm/min-plus-convolution/src/lib.rs
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yuki952/src/main.rs
documentation_of: crates/algorithm/monotone-minima/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/monotone-minima/src/lib.rs
- /library/crates/algorithm/monotone-minima/src/lib.rs.html
title: crates/algorithm/monotone-minima/src/lib.rs
---
