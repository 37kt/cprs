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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "fn dfs(\n    x1: usize,\n    x2: usize,\n    y1: usize,\n    y2: usize,\n\
    \    select: &impl Fn(usize, usize, usize) -> bool,\n    res: &mut [usize],\n\
    ) {\n    if x1 == x2 {\n        return;\n    }\n    let x = (x1 + x2) / 2;\n \
    \   let mut best_y = y1;\n    for y in y1 + 1..y2 {\n        if select(x, best_y,\
    \ y) {\n            best_y = y;\n        }\n    }\n    res[x] = best_y;\n    dfs(x1,\
    \ x, y1, best_y + 1, select, res);\n    dfs(x + 1, x2, best_y, y2, select, res);\n\
    }\n\n/// select(i, j, k): f(i, j) >= f(i, k)\npub fn monotone_minima(\n    h:\
    \ usize,\n    w: usize,\n    select: impl Fn(usize, usize, usize) -> bool,\n)\
    \ -> Vec<usize> {\n    let mut res = vec![0; h];\n    dfs(0, h, 0, w, &select,\
    \ &mut res);\n    res\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/monotone-minima/src/lib.rs
  requiredBy:
  - crates/algorithm/min-plus-convolution/src/lib.rs
  timestamp: '2024-04-07 09:46:12+09:00'
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
