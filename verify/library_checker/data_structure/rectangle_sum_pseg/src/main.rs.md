---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/add.rs
    title: crates/algebra/algebraic_structure/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/affine.rs
    title: crates/algebra/algebraic_structure/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/count_sum.rs
    title: crates/algebra/algebraic_structure/src/count_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/countsum_affine.rs
    title: crates/algebra/algebraic_structure/src/countsum_affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/max.rs
    title: crates/algebra/algebraic_structure/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/min.rs
    title: crates/algebra/algebraic_structure/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/mul.rs
    title: crates/algebra/algebraic_structure/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/semiring.rs
    title: crates/algebra/algebraic_structure/src/semiring.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/trivial_group.rs
    title: crates/algebra/algebraic_structure/src/trivial_group.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/xor.rs
    title: crates/algebra/algebraic_structure/src/xor.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/rectangle_sum\n\
    \nuse algebraic_structure::magma::AddOperator;\nuse persistent_segment_tree::PersistentSegmentTree;\n\
    use proconio::fastout;\nuse proconio::input;\n\nstruct Add {\n    x: usize,\n\
    \    y: usize,\n    w: i64,\n}\n\n#[fastout]\nfn main() {\n    input! {\n    \
    \    n: usize,\n        q: usize,\n    }\n    let mut add = vec![];\n    for _\
    \ in 0..n {\n        input! {\n            x: usize,\n            y: usize,\n\
    \            w: i64,\n        }\n        add.push(Add { x, y, w });\n    }\n \
    \   add.sort_by_key(|a| a.x);\n\n    let mut segs = vec![PersistentSegmentTree::<AddOperator<i64>>::new(\n\
    \        1_000_000_000,\n    )];\n    for i in 0..n {\n        let Add { y, w,\
    \ .. } = add[i];\n        segs.push(segs[i].add(y, w));\n    }\n\n    for _ in\
    \ 0..q {\n        input! {\n            xl: usize,\n            yl: usize,\n \
    \           xr: usize,\n            yr: usize,\n        }\n        let xl = add.partition_point(|a|\
    \ a.x < xl);\n        let xr = add.partition_point(|a| a.x < xr);\n        let\
    \ res = segs[xr].fold(yl..yr) - segs[xl].fold(yl..yr);\n        println!(\"{}\"\
    , res);\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_structure/src/add.rs
  - crates/algebra/algebraic_structure/src/affine.rs
  - crates/algebra/algebraic_structure/src/count_sum.rs
  - crates/algebra/algebraic_structure/src/countsum_affine.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/algebra/algebraic_structure/src/max.rs
  - crates/algebra/algebraic_structure/src/min.rs
  - crates/algebra/algebraic_structure/src/mul.rs
  - crates/algebra/algebraic_structure/src/semiring.rs
  - crates/algebra/algebraic_structure/src/trivial_group.rs
  - crates/algebra/algebraic_structure/src/xor.rs
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/persistent_segment_tree/src/node.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
  requiredBy: []
  timestamp: '2025-04-30 05:56:56+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
- /verify/verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs.html
title: verify/library_checker/data_structure/rectangle_sum_pseg/src/main.rs
---
