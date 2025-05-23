---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
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
    PROBLEM: https://judge.yosupo.jp/problem/point_set_range_frequency
    links:
    - https://judge.yosupo.jp/problem/point_set_range_frequency
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_frequency\n\
    \nuse fenwick_tree_01::FenwickTree01;\nuse proconio::fastout;\nuse proconio::input;\n\
    use wavelet_matrix::WaveletMatrix2D;\n\nenum Query {\n    Set(usize, usize),\n\
    \    Query(usize, usize, usize),\n}\n\n#[fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n        a: [usize; n],\n    }\n    let\
    \ mut z = vec![];\n    for (i, &x) in a.iter().enumerate() {\n        z.push((i,\
    \ x));\n    }\n    let mut qs = vec![];\n    for _ in 0..q {\n        input! {\n\
    \            t: usize,\n        }\n        if t == 0 {\n            input! {\n\
    \                k: usize,\n                v: usize,\n            }\n       \
    \     z.push((k, v));\n            qs.push(Query::Set(k, v));\n        } else\
    \ {\n            input! {\n                l: usize,\n                r: usize,\n\
    \                x: usize,\n            }\n            qs.push(Query::Query(l,\
    \ r, x));\n        }\n    }\n\n    let m = z.len();\n    let (wm, mut ft) =\n\
    \        WaveletMatrix2D::<usize, usize, true, false, true>::new_2d_with_containers(z,\
    \ |ord| {\n            FenwickTree01::from_fn(m, |i| if ord[i] < n { 1 } else\
    \ { 0 })\n        });\n\n    let mut b = (0..n).collect::<Vec<_>>();\n    let\
    \ mut i = n;\n    for q in qs {\n        match q {\n            Query::Set(k,\
    \ _) => {\n                wm.access_with(b[k], |i, j| {\n                   \
    \ ft[i].set(j, 0);\n                });\n                b[k] = i;\n         \
    \       wm.access_with(b[k], |i, j| {\n                    ft[i].set(j, 1);\n\
    \                });\n                i += 1;\n            }\n            Query::Query(l,\
    \ r, x) => {\n                let mut res = 0;\n                wm.count_with(l..r,\
    \ x..=x, |i, range, inv| {\n                    if inv {\n                   \
    \     res -= ft[i].fold(range);\n                    } else {\n              \
    \          res += ft[i].fold(range);\n                    }\n                });\n\
    \                println!(\"{}\", res);\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
  requiredBy: []
  timestamp: '2025-05-23 03:46:52+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
- /verify/verify/library_checker/data_structure/point_set_range_frequency/src/main.rs.html
title: verify/library_checker/data_structure/point_set_range_frequency/src/main.rs
---
