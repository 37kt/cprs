---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/disjoint-sparse-table/src/lib.rs
    title: crates/data-structure/disjoint-sparse-table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/sliding-window-aggregation/src/lib.rs
    title: crates/data-structure/sliding-window-aggregation/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/deque_operate_all_composite/src/main.rs
    title: verify/deque_operate_all_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/queue_operate_all_composite/src/main.rs
    title: verify/queue_operate_all_composite/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/staticrmq/src/main.rs
    title: verify/staticrmq/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Monoid {\n    type S;\n    fn e() -> Self::S;\n    fn op(x: &Self::S,\
    \ y: &Self::S) -> Self::S;\n}\n\npub trait ActMonoid: Monoid {\n    type X;\n\
    \    fn act(f: &Self::S, x: &Self::X) -> Self::X;\n}\n\n#[macro_export]\nmacro_rules!\
    \ monoid {\n    ( $ident:ident, $ty:ty, $e:expr, $op:expr ) => {\n        enum\
    \ $ident {}\n        impl Monoid for $ident {\n            type S = $ty;\n   \
    \         fn e() -> $ty {\n                $e\n            }\n            fn op(x:\
    \ &$ty, y: &$ty) -> $ty {\n                $op(x, y)\n            }\n        }\n\
    \    };\n}\n\n#[macro_export]\nmacro_rules! act_monoid {\n    ( $ident:ident,\
    \ $f_ty:ty, $x_ty:ty, $e:expr, $op:expr, $act:expr ) => {\n        monoid!($ident,\
    \ $f_ty, $e, $op);\n        impl ActMonoid for $ident\n        where\n       \
    \     $ident: Monoid,\n        {\n            type X = $x_ty;\n            fn\
    \ act(f: &$f_ty, x: &$x_ty) -> $x_ty {\n                $act(f, x)\n         \
    \   }\n        }\n    };\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algebraic/algebraic/src/lib.rs
  requiredBy:
  - crates/data-structure/disjoint-sparse-table/src/lib.rs
  - crates/data-structure/sliding-window-aggregation/src/lib.rs
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/deque_operate_all_composite/src/main.rs
  - verify/queue_operate_all_composite/src/main.rs
  - verify/staticrmq/src/main.rs
documentation_of: crates/algebraic/algebraic/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebraic/algebraic/src/lib.rs
- /library/crates/algebraic/algebraic/src/lib.rs.html
title: crates/algebraic/algebraic/src/lib.rs
---