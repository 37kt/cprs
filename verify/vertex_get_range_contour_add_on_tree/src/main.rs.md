---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
    title: crates/data-structure/vertex-get-range-contour-add/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_get_range_contour_add_on_tree
    links:
    - https://judge.yosupo.jp/problem/vertex_get_range_contour_add_on_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_get_range_contour_add_on_tree\n\
    \nuse algebraic::{algebra, monoid};\nuse proconio::input;\nuse vertex_get_range_contour_add::VertexGetRangeContourAdd;\n\
    \nalgebra!(M, i64);\nmonoid!(M, 0, |x, y| x + y);\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [i64;\
    \ n],\n        uv: [(usize, usize); n - 1],\n    }\n    let mut rcq = VertexGetRangeContourAdd::<M>::new(&a,\
    \ &uv);\n    for _ in 0..q {\n        input! {\n            t: usize,\n      \
    \      p: usize,\n        }\n        if t == 0 {\n            input! {\n     \
    \           l: usize,\n                r: usize,\n                x: i64,\n  \
    \          }\n            rcq.apply_range(p, l, r, x);\n        } else {\n   \
    \         println!(\"{}\", rcq.get(p));\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/vertex-get-range-contour-add/src/lib.rs
  isVerificationFile: true
  path: verify/vertex_get_range_contour_add_on_tree/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/vertex_get_range_contour_add_on_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/vertex_get_range_contour_add_on_tree/src/main.rs
- /verify/verify/vertex_get_range_contour_add_on_tree/src/main.rs.html
title: verify/vertex_get_range_contour_add_on_tree/src/main.rs
---
