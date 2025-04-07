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
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/point_add_rectangle_sum
    links:
    - https://judge.yosupo.jp/problem/point_add_rectangle_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_rectangle_sum\n\
    \nuse algebraic_structure::magma::AddOperator;\nuse fenwick_tree::FenwickTree;\n\
    use proconio::fastout;\nuse proconio::input;\nuse proconio::read_value;\nuse wavelet_matrix::WaveletMatrix2D;\n\
    \nenum Query {\n    Add(usize, i64),\n    Sum(u32, u32, u32, u32),\n}\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n\n   \
    \ let mut qs = vec![];\n    let mut ps = vec![];\n    let mut ws = vec![];\n \
    \   for qi in 0..n + q {\n        let t = if qi < n { 0 } else { read_value!(usize)\
    \ };\n        if t == 0 {\n            input! {\n                x: u32,\n   \
    \             y: u32,\n                w: i64,\n            }\n            let\
    \ i = ps.len();\n            ps.push((x, y));\n            if qi < n {\n     \
    \           ws.push(w);\n            } else {\n                qs.push(Query::Add(i,\
    \ w));\n                ws.push(0);\n            }\n        } else {\n       \
    \     input! {\n                xl: u32,\n                yl: u32,\n         \
    \       xr: u32,\n                yr: u32,\n            }\n            qs.push(Query::Sum(xl,\
    \ yl, xr, yr));\n        }\n    }\n\n    let (wm, mut bits) =\n        WaveletMatrix2D::<_,\
    \ _, false, false, true>::new_2d_with_containers(&ps, |idx| {\n            FenwickTree::<AddOperator<i64>>::from_iter(idx.iter().map(|&i|\
    \ ws[i]))\n        });\n\n    for q in qs {\n        match q {\n            Query::Add(i,\
    \ w) => {\n                wm.access_with(i, |d, i| {\n                    bits[d].add(i,\
    \ w);\n                });\n            }\n            Query::Sum(xl, yl, xr,\
    \ yr) => {\n                let mut s = 0;\n                wm.count_with(xl..xr,\
    \ yl..yr, |d, range, inv| {\n                    let t = bits[d].fold(range);\n\
    \                    s += if inv { -t } else { t };\n                });\n   \
    \             println!(\"{}\", s);\n            }\n        }\n    }\n}\n"
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
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-04-07 08:03:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
- /verify/verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs.html
title: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
---
