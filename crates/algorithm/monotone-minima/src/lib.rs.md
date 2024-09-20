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
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/monotone-minima/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/monotone-minima/src/lib.rs
- /library/crates/algorithm/monotone-minima/src/lib.rs.html
title: crates/algorithm/monotone-minima/src/lib.rs
---
