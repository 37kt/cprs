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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Algebra {\n    type S;\n}\n\npub trait Act: Algebra {\n    type\
    \ X;\n    fn act(f: &Self::S, x: &Self::X) -> Self::X;\n}\n\npub trait Monoid:\
    \ Algebra {\n    fn e() -> Self::S;\n    fn op(x: &Self::S, y: &Self::S) -> Self::S;\n\
    }\n\npub trait Group: Monoid {\n    fn inv(x: &Self::S) -> Self::S;\n}\n\npub\
    \ trait Zero {\n    fn zero() -> Self;\n    fn is_zero(&self) -> bool;\n}\n\n\
    pub trait One {\n    fn one() -> Self;\n    fn is_one(&self) -> bool;\n}\n\n#[macro_export]\n\
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
    \     $inv(x)\n            }\n        }\n    };\n}\n\nmacro_rules! impl_zero_one\
    \ {\n    ($($t:ty)*) => {\n        $(\n            impl $crate::Zero for $t {\n\
    \                fn zero() -> Self {\n                    0\n                }\n\
    \                fn is_zero(&self) -> bool {\n                    *self == 0\n\
    \                }\n            }\n            impl $crate::One for $t {\n   \
    \             fn one() -> Self {\n                    1\n                }\n \
    \               fn is_one(&self) -> bool {\n                    *self == 1\n \
    \               }\n            }\n        )*\n    };\n}\n\nimpl_zero_one!(usize\
    \ u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);\n"
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
