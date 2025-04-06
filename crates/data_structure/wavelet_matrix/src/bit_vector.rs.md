---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
    title: verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/range_kth_smallest/src/main.rs
    title: verify/library_checker/data_structure/range_kth_smallest/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
    title: verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/rectangle_sum/src/main.rs
    title: verify/library_checker/data_structure/rectangle_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_frequency/src/main.rs
    title: verify/library_checker/data_structure/static_range_frequency/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use numeric_traits::Integer;\n\nconst W: usize = 64;\n\n// \u3059\u3079\u3066\
    \u306Eset\u3092\u7D42\u3048\u305F\u5F8Cbuild\u3092\u547C\u3076\u3068\u30AF\u30A8\
    \u30EA\u3092\u6295\u3052\u3089\u308C\u308B\u3088\u3046\u306B\u306A\u308B\n#[derive(Clone)]\n\
    pub(crate) struct BitVector {\n    n: usize,\n    bit: Vec<u64>,\n    sum: Vec<u32>,\n\
    }\n\nimpl BitVector {\n    pub(crate) fn new(n: usize) -> Self {\n        let\
    \ sz = (n + 1).ceil_div(W);\n        Self {\n            n,\n            bit:\
    \ vec![0; sz],\n            sum: vec![0; sz + 1],\n        }\n    }\n\n    pub(crate)\
    \ fn set(&mut self, i: usize) {\n        assert!(i < self.n);\n        self.bit[i\
    \ / W] |= 1 << (i % W);\n    }\n\n    pub(crate) fn build(&mut self) {\n     \
    \   for i in 0..self.bit.len() {\n            self.sum[i + 1] = self.sum[i] +\
    \ self.bit[i].count_ones();\n        }\n    }\n\n    pub(crate) fn get(&self,\
    \ i: usize) -> bool {\n        (self.bit[i / W] >> (i % W)) & 1 != 0\n    }\n\n\
    \    pub(crate) fn count_prefix(&self, i: usize, f: bool) -> usize {\n       \
    \ let cnt =\n            (self.sum[i / W] + (self.bit[i / W] & ((1 << (i % W))\
    \ - 1)).count_ones()) as usize;\n        if f {\n            cnt\n        } else\
    \ {\n            i - cnt\n        }\n    }\n}\n"
  dependsOn:
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  requiredBy:
  - crates/data_structure/wavelet_matrix/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - verify/library_checker/data_structure/static_range_frequency/src/main.rs
  - verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
  - verify/library_checker/data_structure/range_kth_smallest/src/main.rs
  - verify/library_checker/data_structure/rectangle_sum/src/main.rs
documentation_of: crates/data_structure/wavelet_matrix/src/bit_vector.rs
layout: document
redirect_from:
- /library/crates/data_structure/wavelet_matrix/src/bit_vector.rs
- /library/crates/data_structure/wavelet_matrix/src/bit_vector.rs.html
title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
---
