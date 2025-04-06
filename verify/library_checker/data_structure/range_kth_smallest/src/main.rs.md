---
data:
  _extendedDependsOn:
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
    PROBLEM: https://judge.yosupo.jp/problem/range_kth_smallest
    links:
    - https://judge.yosupo.jp/problem/range_kth_smallest
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_kth_smallest\n\
    \nuse proconio::fastout;\nuse proconio::input;\nuse wavelet_matrix::WaveletMatrix;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [u32; n],\n    }\n\n    let wm = WaveletMatrix::<_>::new(a);\n   \
    \ for _ in 0..q {\n        input! {\n            l: usize,\n            r: usize,\n\
    \            k: usize,\n        }\n        println!(\"{}\", wm.kth_smallest(l..r,\
    \ k).unwrap());\n    }\n}\n"
  dependsOn:
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/range_kth_smallest/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/range_kth_smallest/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/range_kth_smallest/src/main.rs
- /verify/verify/library_checker/data_structure/range_kth_smallest/src/main.rs.html
title: verify/library_checker/data_structure/range_kth_smallest/src/main.rs
---
