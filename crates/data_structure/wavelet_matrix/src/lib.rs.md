---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/macros.rs
    title: crates/algebra/algebraic_traits/src/macros.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
  - icon: ':warning:'
    path: crates/misc/as_half_open_range/src/lib.rs
    title: crates/misc/as_half_open_range/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/coordinate_compression/src/lib.rs
    title: crates/misc/coordinate_compression/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/bit_vector.rs
    title: crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/internal.rs
    title: crates/data_structure/wavelet_matrix/src/internal.rs
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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    borrow::Borrow,\n    ops::{Range, RangeBounds},\n};\n\nuse\
    \ as_half_open_range::AsHalfOpenRange;\nuse bit_vector::BitVector;\nuse coordinate_compression::CoordinateCompression;\n\
    use numeric_traits::{Cast, Inf, Integer, NegInf};\n\nmod bit_vector;\nmod internal;\n\
    \npub type WaveletMatrix<Y, const Y_SMALL: bool = false, const INVERTIVE: bool\
    \ = false> =\n    WaveletMatrix2D<usize, Y, true, Y_SMALL, INVERTIVE>;\n\nimpl<Y,\
    \ const Y_SMALL: bool, const INVERTIVE: bool> WaveletMatrix<Y, Y_SMALL, INVERTIVE>\n\
    where\n    Y: Integer + Inf + NegInf + Cast<usize>,\n    u32: Cast<Y>,\n{\n  \
    \  pub fn new<I>(a: I) -> Self\n    where\n        I: IntoIterator,\n        I::Item:\
    \ Borrow<Y>,\n    {\n        Self::new_with_containers(a, |_| ()).0\n    }\n\n\
    \    pub fn new_with_containers<Container, I>(\n        a: I,\n        build_container:\
    \ impl FnMut(&[usize]) -> Container,\n    ) -> (Self, Vec<Container>)\n    where\n\
    \        I: IntoIterator,\n        I::Item: Borrow<Y>,\n    {\n        WaveletMatrix2D::<usize,\
    \ Y, true, Y_SMALL, INVERTIVE>::new_2d_with_containers(\n            a.into_iter().enumerate().map(|(i,\
    \ y)| (i, *y.borrow())),\n            build_container,\n        )\n    }\n}\n\n\
    pub struct WaveletMatrix2D<\n    X,\n    Y,\n    const X_SMALL: bool = false,\n\
    \    const Y_SMALL: bool = false,\n    const INVERTIVE: bool = false,\n> where\n\
    \    X: Integer + Inf + NegInf + Cast<usize>,\n    Y: Integer + Inf + NegInf +\
    \ Cast<usize>,\n    u32: Cast<X>,\n    u32: Cast<Y>,\n{\n    n: usize,\n    m:\
    \ usize,\n    lg: usize,\n    ccx: CoordinateCompression<X, X_SMALL, true>,\n\
    \    ccy: CoordinateCompression<Y, Y_SMALL, false>,\n    pos: Vec<usize>,\n  \
    \  mid: Vec<usize>,\n    bv: Vec<BitVector>,\n}\n\nimpl<X, Y, const X_SMALL: bool,\
    \ const Y_SMALL: bool, const INVERTIVE: bool>\n    WaveletMatrix2D<X, Y, X_SMALL,\
    \ Y_SMALL, INVERTIVE>\nwhere\n    X: Integer + Inf + NegInf + Cast<usize>,\n \
    \   Y: Integer + Inf + NegInf + Cast<usize>,\n    u32: Cast<X>,\n    u32: Cast<Y>,\n\
    {\n    pub fn new_2d<I>(ps: I) -> Self\n    where\n        I: IntoIterator,\n\
    \        I::Item: Borrow<(X, Y)>,\n    {\n        Self::new_2d_with_containers(ps,\
    \ |_| ()).0\n    }\n\n    pub fn new_2d_with_containers<Container, I>(\n     \
    \   ps: I,\n        mut build_container: impl FnMut(&[usize]) -> Container,\n\
    \    ) -> (Self, Vec<Container>)\n    where\n        I: IntoIterator,\n      \
    \  I::Item: Borrow<(X, Y)>,\n    {\n        let ps = ps.into_iter().map(|p| *p.borrow()).collect::<Vec<_>>();\n\
    \        let n = ps.len();\n\n        let (ccx, pos) = CoordinateCompression::<X,\
    \ X_SMALL, true>::new(ps.iter().map(|&(x, _)| x));\n        let (ccy, _) = CoordinateCompression::<Y,\
    \ Y_SMALL, false>::new(ps.iter().map(|&(_, y)| y));\n        let m = ccy.len();\n\
    \        let lg = m.checked_ceil_log2().unwrap_or(0);\n\n        let mut a = vec![0;\
    \ n]; // \u30A4\u30F3\u30C7\u30C3\u30AF\u30B9\n        let mut b = vec![0; n];\
    \ // \u30A8\u30F3\u30B3\u30FC\u30C9\u3055\u308C\u305F\u5024\n        for i in\
    \ 0..n {\n            a[pos[i]] = i;\n            b[pos[i]] = ccy.encode(ps[i].1);\n\
    \        }\n\n        let mut mid = vec![0; lg];\n        let mut bv = vec![BitVector::new(n);\
    \ lg];\n        let mut containers = Vec::with_capacity(lg + 1);\n        containers.push(build_container(&a));\n\
    \        let mut a0 = vec![0; n];\n        let mut a1 = vec![0; n];\n        let\
    \ mut b0 = vec![0; n];\n        let mut b1 = vec![0; n];\n        for d in (0..lg).rev()\
    \ {\n            let mut p0 = 0;\n            let mut p1 = 0;\n            for\
    \ i in 0..n {\n                if b[i] >> d & 1 == 0 {\n                    a0[p0]\
    \ = a[i];\n                    b0[p0] = b[i];\n                    p0 += 1;\n\
    \                } else {\n                    bv[d].set(i);\n               \
    \     a1[p1] = a[i];\n                    b1[p1] = b[i];\n                   \
    \ p1 += 1;\n                }\n            }\n            a[..p0].copy_from_slice(&a0[..p0]);\n\
    \            a[p0..].copy_from_slice(&a1[..p1]);\n            b[..p0].copy_from_slice(&b0[..p0]);\n\
    \            b[p0..].copy_from_slice(&b1[..p1]);\n            mid[d] = p0;\n \
    \           bv[d].build();\n            containers.push(build_container(&a));\n\
    \        }\n        containers.reverse();\n\n        (\n            Self {\n \
    \               n,\n                m,\n                lg,\n                ccx,\n\
    \                ccy,\n                pos,\n                mid,\n          \
    \      bv,\n            },\n            containers,\n        )\n    }\n\n    pub\
    \ fn len(&self) -> usize {\n        self.n\n    }\n\n    pub fn is_empty(&self)\
    \ -> bool {\n        self.n == 0\n    }\n\n    pub fn map_index(&self, i: usize)\
    \ -> usize {\n        self.pos[i]\n    }\n\n    pub fn count(&self, x_range: impl\
    \ RangeBounds<X>, y_range: impl RangeBounds<Y>) -> usize {\n        self.count_with(x_range,\
    \ y_range, |_, _, _| {})\n    }\n\n    pub fn count_with(\n        &self,\n  \
    \      x_range: impl RangeBounds<X>,\n        y_range: impl RangeBounds<Y>,\n\
    \        mut f: impl FnMut(usize, Range<usize>, bool),\n    ) -> usize {\n   \
    \     let (xl, xr) = x_range.as_half_open_range(X::neg_inf(), X::inf());\n   \
    \     let (yl, yr) = y_range.as_half_open_range(Y::neg_inf(), Y::inf());\n   \
    \     let xl = self.ccx.encode(xl);\n        let xr = self.ccx.encode(xr);\n \
    \       let yl = self.ccy.encode(yl);\n        let yr = self.ccy.encode(yr);\n\
    \n        self.count_with_(xl, xr, yl, yr, &mut f)\n    }\n\n    pub fn access(&self,\
    \ i: usize) -> Y {\n        self.access_with(i, |_, _| {})\n    }\n\n    pub fn\
    \ access_with(&self, i: usize, mut f: impl FnMut(usize, usize)) -> Y {\n     \
    \   let mut y = 0;\n        let mut i = self.map_index(i);\n        f(self.lg,\
    \ i);\n        for d in (0..self.lg).rev() {\n            if self.bv[d].get(i)\
    \ {\n                y |= 1 << d;\n                i = self.bv[d].count_prefix(i,\
    \ true) + self.mid[d];\n            } else {\n                i = self.bv[d].count_prefix(i,\
    \ false);\n            }\n            f(d, i);\n        }\n        self.ccy.decode(y)\n\
    \    }\n\n    pub fn kth_smallest(&self, x_range: impl RangeBounds<X>, mut k:\
    \ usize) -> Option<Y> {\n        let (xl, xr) = x_range.as_half_open_range(X::neg_inf(),\
    \ X::inf());\n        let mut xl = self.ccx.encode(xl);\n        let mut xr =\
    \ self.ccx.encode(xr);\n        if k >= xr - xl {\n            return None;\n\
    \        }\n\n        let mut y = 0;\n        for d in (0..self.lg).rev() {\n\
    \            let l0 = self.bv[d].count_prefix(xl, false);\n            let r0\
    \ = self.bv[d].count_prefix(xr, false);\n            let l1 = xl + self.mid[d]\
    \ - l0;\n            let r1 = xr + self.mid[d] - r0;\n            if k < r0 -\
    \ l0 {\n                (xl, xr) = (l0, r0);\n            } else {\n         \
    \       k -= r0 - l0;\n                y |= 1 << d;\n                (xl, xr)\
    \ = (l1, r1);\n            }\n        }\n        Some(self.ccy.decode(y))\n  \
    \  }\n\n    pub fn kth_largest(&self, x_range: impl RangeBounds<X>, k: usize)\
    \ -> Option<Y> {\n        let (xl, xr) = x_range.as_half_open_range(X::neg_inf(),\
    \ X::inf());\n        let xl = self.ccx.encode(xl);\n        let xr = self.ccx.encode(xr);\n\
    \        if k >= xr - xl {\n            return None;\n        }\n\n        self.kth_smallest(x_range,\
    \ xr - xl - 1 - k)\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  - crates/misc/as_half_open_range/src/lib.rs
  - crates/misc/coordinate_compression/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/wavelet_matrix/src/lib.rs
  requiredBy:
  - crates/data_structure/wavelet_matrix/src/bit_vector.rs
  - crates/data_structure/wavelet_matrix/src/internal.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/rectangle_sum/src/main.rs
  - verify/library_checker/data_structure/point_add_rectangle_sum/src/main.rs
  - verify/library_checker/data_structure/range_kth_smallest/src/main.rs
  - verify/library_checker/data_structure/static_range_frequency/src/main.rs
  - verify/library_checker/data_structure/rectangle_add_point_get/src/main.rs
documentation_of: crates/data_structure/wavelet_matrix/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/wavelet_matrix/src/lib.rs
- /library/crates/data_structure/wavelet_matrix/src/lib.rs.html
title: crates/data_structure/wavelet_matrix/src/lib.rs
---
