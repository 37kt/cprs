---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/disjoint-sparse-table/src/lib.rs
    title: crates/data-structure/disjoint-sparse-table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/fenwick-tree/src/lib.rs
    title: crates/data-structure/fenwick-tree/src/lib.rs
  - icon: ':x:'
    path: crates/data-structure/lazy-segment-tree/src/lib.rs
    title: crates/data-structure/lazy-segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/segment-tree/src/lib.rs
    title: crates/data-structure/segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/sliding-window-aggregation/src/lib.rs
    title: crates/data-structure/sliding-window-aggregation/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/discrete-logarithm/src/lib.rs
    title: crates/math/discrete-logarithm/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/re-rooting-dp/src/lib.rs
    title: crates/tree/re-rooting-dp/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/deque_operate_all_composite/src/main.rs
    title: verify/deque_operate_all_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/discrete_logarithm_mod/src/main.rs
    title: verify/discrete_logarithm_mod/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/point_add_range_sum/src/main.rs
    title: verify/point_add_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/queue_operate_all_composite/src/main.rs
    title: verify/queue_operate_all_composite/src/main.rs
  - icon: ':x:'
    path: verify/range_affine_range_sum/src/main.rs
    title: verify/range_affine_range_sum/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/static_range_inversions_query/src/main.rs
    title: verify/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/staticrmq/src/main.rs
    title: verify/staticrmq/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/yuki1333/src/main.rs
    title: verify/yuki1333/src/main.rs
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':question:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Algebra {\n    type S;\n}\n\npub trait Act: Algebra {\n    type\
    \ X;\n    fn act(f: &Self::S, x: &Self::X) -> Self::X;\n}\n\npub trait Monoid:\
    \ Algebra {\n    fn e() -> Self::S;\n    fn op(x: &Self::S, y: &Self::S) -> Self::S;\n\
    }\n\npub trait Group: Monoid {\n    fn inv(x: &Self::S) -> Self::S;\n}\n\n#[macro_export]\n\
    macro_rules! algebra {\n    ($ident:ident, $ty:ty) => {\n        #[derive(Clone)]\n\
    \        enum $ident {}\n        impl $crate::Algebra for $ident {\n         \
    \   type S = $ty;\n        }\n    };\n}\n\n#[macro_export]\nmacro_rules! act {\n\
    \    ($ident:ident, $tar:ty, $act:expr) => {\n        impl $crate::Act for $ident\
    \ {\n            type X = $tar;\n            #[inline]\n            fn act(f:\
    \ &Self::S, x: &Self::X) -> Self::X {\n                $act(f, x)\n          \
    \  }\n        }\n    };\n}\n\n#[macro_export]\nmacro_rules! monoid {\n    ($ident:ident,\
    \ $e:expr, $op:expr) => {\n        impl $crate::Monoid for $ident {\n        \
    \    #[inline]\n            fn e() -> Self::S {\n                $e\n        \
    \    }\n            #[inline]\n            fn op(x: &Self::S, y: &Self::S) ->\
    \ Self::S {\n                $op(x, y)\n            }\n        }\n    };\n}\n\n\
    #[macro_export]\nmacro_rules! group {\n    ($ident:ident, $e:expr, $op:expr, $inv:expr)\
    \ => {\n        impl $crate::Monoid for $ident {\n            #[inline]\n    \
    \        fn e() -> Self::S {\n                $e\n            }\n            #[inline]\n\
    \            fn op(x: &Self::S, y: &Self::S) -> Self::S {\n                $op(x,\
    \ y)\n            }\n        }\n        impl $crate::Group for $ident {\n    \
    \        #[inline]\n            fn inv(x: &Self::S) -> Self::S {\n           \
    \     $inv(x)\n            }\n        }\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algebraic/algebraic/src/lib.rs
  requiredBy:
  - crates/tree/re-rooting-dp/src/lib.rs
  - crates/math/discrete-logarithm/src/lib.rs
  - crates/data-structure/lazy-segment-tree/src/lib.rs
  - crates/data-structure/segment-tree/src/lib.rs
  - crates/data-structure/disjoint-sparse-table/src/lib.rs
  - crates/data-structure/fenwick-tree/src/lib.rs
  - crates/data-structure/sliding-window-aggregation/src/lib.rs
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: LIBRARY_SOME_WA
  verifiedWith:
  - verify/static_range_inversions_query/src/main.rs
  - verify/deque_operate_all_composite/src/main.rs
  - verify/queue_operate_all_composite/src/main.rs
  - verify/staticrmq/src/main.rs
  - verify/range_affine_range_sum/src/main.rs
  - verify/discrete_logarithm_mod/src/main.rs
  - verify/point_add_range_sum/src/main.rs
  - verify/yuki1333/src/main.rs
documentation_of: crates/algebraic/algebraic/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebraic/algebraic/src/lib.rs
- /library/crates/algebraic/algebraic/src/lib.rs.html
title: crates/algebraic/algebraic/src/lib.rs
---
