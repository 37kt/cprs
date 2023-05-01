---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
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
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algebraic/algebraic/src/lib.rs
layout: document
redirect_from:
- /library/crates/algebraic/algebraic/src/lib.rs
- /library/crates/algebraic/algebraic/src/lib.rs.html
title: crates/algebraic/algebraic/src/lib.rs
---
