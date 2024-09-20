---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/rectangle_add_point_get
    links:
    - https://judge.yosupo.jp/problem/rectangle_add_point_get
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_add_point_get\n\
    \nuse algebraic::{algebra, monoid};\nuse dual_range_tree::DualRangeTree;\nuse\
    \ proconio::input;\n\nalgebra!(F, usize);\nmonoid!(F, 0, |a, b| a + b);\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n    let\
    \ mut qs = vec![];\n    for _ in 0..n {\n        input! {\n            l: usize,\n\
    \            d: usize,\n            r: usize,\n            u: usize,\n       \
    \     w: usize,\n        }\n        qs.push((0, l, d, r, u, w));\n    }\n    let\
    \ mut ps = vec![];\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \        }\n        if t == 0 {\n            input! {\n                l: usize,\n\
    \                d: usize,\n                r: usize,\n                u: usize,\n\
    \                w: usize,\n            }\n            qs.push((0, l, d, r, u,\
    \ w));\n        } else {\n            input! {\n                x: usize,\n  \
    \              y: usize,\n            }\n            qs.push((1, x, y, 0, 0, 0));\n\
    \            ps.push((x, y));\n        }\n    }\n    let mut seg = DualRangeTree::<usize,\
    \ F>::new(ps);\n    for (t, l, d, r, u, w) in qs {\n        if t == 0 {\n    \
    \        seg.apply_range(l..r, d..u, w);\n        } else {\n            println!(\"\
    {}\", seg.get((l, d)));\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/rectangle_add_point_get/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/rectangle_add_point_get/src/main.rs
layout: document
redirect_from:
- /verify/verify/rectangle_add_point_get/src/main.rs
- /verify/verify/rectangle_add_point_get/src/main.rs.html
title: verify/rectangle_add_point_get/src/main.rs
---
