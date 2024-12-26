---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
    title: crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_add_rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/point_add_rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum\n\
    \nuse algebraic::{algebra, monoid};\nuse proconio::input;\nuse segment_tree_on_wavelet_matrix::SegmentTreeOnWaveletMatrix;\n\
    \nalgebra!(M, i64);\nmonoid!(M, 0, |&a, &b| a + b);\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        xyw:\
    \ [(i64, i64, i64); n],\n    }\n    let mut xy: Vec<_> = xyw.iter().map(|&(x,\
    \ y, _)| (x, y)).collect();\n    let mut qs = vec![];\n    for _ in 0..q {\n \
    \       input! {\n            ty: usize,\n            a: [i64; 3 + ty],\n    \
    \    }\n        if ty == 0 {\n            xy.push((a[0], a[1]));\n        }\n\
    \        qs.push(a);\n    }\n    let mut rt = SegmentTreeOnWaveletMatrix::<i64,\
    \ M>::new(xy);\n    for &(x, y, w) in &xyw {\n        rt.add((x, y), w);\n   \
    \ }\n    for a in qs {\n        if a.len() == 3 {\n            rt.add((a[0], a[1]),\
    \ a[2]);\n        } else {\n            let res = rt.prod(a[0]..a[2], a[1]..a[3]);\n\
    \            println!(\"{}\", res);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/segment-tree-on-wavelet-matrix/src/lib.rs
  isVerificationFile: true
  path: verify/point_add_rectangle_sum_2/src/main.rs
  requiredBy: []
  timestamp: '2024-12-26 06:54:01+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/point_add_rectangle_sum_2/src/main.rs
layout: document
redirect_from:
- /verify/verify/point_add_rectangle_sum_2/src/main.rs
- /verify/verify/point_add_rectangle_sum_2/src/main.rs.html
title: verify/point_add_rectangle_sum_2/src/main.rs
---
