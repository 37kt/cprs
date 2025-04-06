---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
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
  code: "use std::ops::Range;\n\nuse numeric_traits::{Cast, Inf, Integer, NegInf};\n\
    \nuse crate::WaveletMatrix2D;\n\nimpl<X, Y, const X_SMALL: bool, const Y_SMALL:\
    \ bool, const INVERTIVE: bool>\n    WaveletMatrix2D<X, Y, X_SMALL, Y_SMALL, INVERTIVE>\n\
    where\n    X: Integer + Inf + NegInf + Cast<usize>,\n    Y: Integer + Inf + NegInf\
    \ + Cast<usize>,\n    u32: Cast<X>,\n    u32: Cast<Y>,\n{\n    pub(crate) fn count_with_(\n\
    \        &self,\n        xl: usize,\n        xr: usize,\n        yl: usize,\n\
    \        yr: usize,\n        mut f: impl FnMut(usize, Range<usize>, bool),\n \
    \   ) -> usize {\n        if INVERTIVE {\n            self.count_prefix_with_(xl,\
    \ xr, yr, &mut f)\n                - self.count_prefix_with_(xl, xr, yl, |d, x,\
    \ inv| f(d, x, !inv))\n        } else {\n            self.dfs_count_with_(self.lg,\
    \ xl, xr, yl, yr, 0, 1 << self.lg, &mut f)\n        }\n    }\n\n    #[allow(clippy::too_many_arguments)]\n\
    \    fn dfs_count_with_(\n        &self,\n        d: usize,\n        xl: usize,\n\
    \        xr: usize,\n        yl: usize,\n        yr: usize,\n        a: usize,\n\
    \        b: usize,\n        f: &mut impl FnMut(usize, Range<usize>, bool),\n \
    \   ) -> usize {\n        if yr <= a || b <= yl {\n            return 0;\n   \
    \     } else if yl <= a && b <= yr {\n            f(d, xl..xr, false);\n     \
    \       return xr - xl;\n        }\n        let d = d - 1;\n        let c = (a\
    \ + b) >> 1;\n        let l0 = self.bv[d].count_prefix(xl, false);\n        let\
    \ r0 = self.bv[d].count_prefix(xr, false);\n        let l1 = xl + self.mid[d]\
    \ - l0;\n        let r1 = xr + self.mid[d] - r0;\n        self.dfs_count_with_(d,\
    \ l0, r0, yl, yr, a, c, f)\n            + self.dfs_count_with_(d, l1, r1, yl,\
    \ yr, c, b, f)\n    }\n\n    fn count_prefix_with_(\n        &self,\n        mut\
    \ xl: usize,\n        mut xr: usize,\n        y: usize,\n        mut f: impl FnMut(usize,\
    \ Range<usize>, bool),\n    ) -> usize {\n        if xl == xr || y == 0 {\n  \
    \          return 0;\n        } else if y == self.m {\n            f(self.lg,\
    \ xl..xr, false);\n            return xr - xl;\n        }\n\n        let mut cnt\
    \ = 0;\n        for d in (0..self.lg).rev() {\n            let l0 = self.bv[d].count_prefix(xl,\
    \ false);\n            let r0 = self.bv[d].count_prefix(xr, false);\n        \
    \    let l1 = xl + self.mid[d] - l0;\n            let r1 = xr + self.mid[d] -\
    \ r0;\n            if y >> d & 1 == 0 {\n                (xl, xr) = (l0, r0);\n\
    \            } else {\n                f(d, l0..r0, false);\n                cnt\
    \ += r0 - l0;\n                (xl, xr) = (l1, r1);\n            }\n        }\n\
    \        cnt\n    }\n}\n"
  dependsOn:
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/wavelet_matrix/src/internal.rs
  requiredBy:
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - verify/library_checker/data_structure/static_range_frequency/src/main.rs
  - verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
  - verify/library_checker/data_structure/range_kth_smallest/src/main.rs
  - verify/library_checker/data_structure/rectangle_sum/src/main.rs
documentation_of: crates/data_structure/wavelet_matrix/src/internal.rs
layout: document
redirect_from:
- /library/crates/data_structure/wavelet_matrix/src/internal.rs
- /library/crates/data_structure/wavelet_matrix/src/internal.rs.html
title: crates/data_structure/wavelet_matrix/src/internal.rs
---
