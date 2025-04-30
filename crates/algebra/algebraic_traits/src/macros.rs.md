---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/lib.rs
    title: crates/algebra/algebraic_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_traits/src/traits.rs
    title: crates/algebra/algebraic_traits/src/traits.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_and_convolution/src/lib.rs
    title: crates/convolution/bitwise_and_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_or_convolution/src/lib.rs
    title: crates/convolution/bitwise_or_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/bitwise_xor_convolution/src/lib.rs
    title: crates/convolution/bitwise_xor_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/convolution/src/lib.rs
    title: crates/convolution/convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/gcd_convolution/src/lib.rs
    title: crates/convolution/gcd_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/lcm_convolution/src/lib.rs
    title: crates/convolution/lcm_convolution/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/disjoint_sparse_table/src/lib.rs
    title: crates/data_structure/disjoint_sparse_table/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/foldable_deque/src/lib.rs
    title: crates/data_structure/foldable_deque/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - icon: ':warning:'
    path: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/union_find/union_find/src/lib.rs
    title: crates/data_structure/union_find/union_find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/linear_algebra/matrix/src/lib.rs
    title: crates/linear_algebra/matrix/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling_hash/src/lib.rs
    title: crates/string/rolling_hash/src/lib.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
    title: verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
    title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
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
  code: "// define_algebra! {}\n/// usage:\n/// ```rust\n/// use algebraic_traits::define_algebra;\n\
    /// define_algebra! {\n///     name: MyAlgebra,\n///     value: i32,\n///    \
    \ op: |x: &i32, y: &i32| x + y,\n///     unit: 0,\n///     inv: |x: &i32| -x,\n\
    ///     associative,\n///     commutative,\n///     idempotent,\n/// }\n/// ```\n\
    #[macro_export]\nmacro_rules! define_algebra {\n    (name: $name:ident, value:\
    \ $value:ty) => {\n        enum $name {}\n        impl $crate::Algebraic for $name\
    \ {\n            type Value = $value;\n        }\n    };\n\n    ($vis:vis, name:\
    \ $name:ident, value: $value:ty) => {\n        $vis enum $name {}\n        impl\
    \ $crate::Algebraic for $name {\n            type Value = $value;\n        }\n\
    \    };\n\n    (name: $name:ident, value: $value:ty, $($rest:tt)*) => {\n    \
    \    define_algebra!(name: $name, value: $value);\n        define_algebra!(@impl\
    \ $name, $($rest)*);\n    };\n\n    ($vis:vis, name: $name:ident, value: $value:ty,\
    \ $($rest:tt)*) => {\n        define_algebra!($vis, name: $name, value: $value);\n\
    \        define_algebra!(@impl $name, $($rest)*);\n    };\n\n    (@impl $name:ident,\
    \ op: $op:expr, $($rest:tt)*) => {\n        impl $crate::Magma for $name {\n \
    \           fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {\n       \
    \         $op(x, y)\n            }\n        }\n        define_algebra!(@impl $name,\
    \ $($rest)*);\n    };\n\n    (@impl $name:ident, unit: $unit:expr, $($rest:tt)*)\
    \ => {\n        impl $crate::Unital for $name {\n            fn unit() -> Self::Value\
    \ {\n                $unit\n            }\n        }\n        define_algebra!(@impl\
    \ $name, $($rest)*);\n    };\n\n    (@impl $name:ident, inv: $inv:expr, $($rest:tt)*)\
    \ => {\n        impl $crate::Invertive for $name {\n            fn inv(x: &Self::Value)\
    \ -> Self::Value {\n                $inv(x)\n            }\n        }\n      \
    \  define_algebra!(@impl $name, $($rest)*);\n    };\n\n    (@impl $name:ident,\
    \ associative, $($rest:tt)*) => {\n        impl $crate::Associative for $name\
    \ {}\n        define_algebra!(@impl $name, $($rest)*);\n    };\n\n    (@impl $name:ident,\
    \ commutative, $($rest:tt)*) => {\n        impl $crate::Commutative for $name\
    \ {}\n        define_algebra!(@impl $name, $($rest)*);\n    };\n\n    (@impl $name:ident,\
    \ idempotent, $($rest:tt)*) => {\n        impl $crate::Idempotent for $name {}\n\
    \        define_algebra!(@impl $name, $($rest)*);\n    };\n\n    (@impl $name:ident\
    \ $(,)?) => {};\n}\n\n// define_act! {}\n/// usage:\n/// ```rust\n/// use algebraic_traits::define_act;\n\
    /// define_act! {\n///     name: MyAct,\n///     operand: MyAlgebra1,\n///   \
    \  operator: MyAlgebra2,\n///     act: |x: &MyAlgebra1::Value, f: &MyAlgebra2::Value|\
    \ x * f,\n/// }\n/// ```\n#[macro_export]\nmacro_rules! define_act {\n    (name:\
    \ $name:ident, operand: $operand:ty, operator: $operator:ty, act: $act:expr $(,)*)\
    \ => {\n        enum $name {}\n        impl $crate::Act for $name {\n        \
    \    type Operand = $operand;\n            type Operator = $operator;\n      \
    \      fn act(\n                x: &<Self::Operand as $crate::Algebraic>::Value,\n\
    \                f: &<Self::Operator as $crate::Algebraic>::Value,\n         \
    \   ) -> <Self::Operand as $crate::Algebraic>::Value {\n                $act(x,\
    \ f)\n            }\n        }\n    };\n\n    ($vis:vis, name: $name:ident, operand:\
    \ $operand:ty, operator: $operator:ty, act: $act:expr $(,)*) => {\n        $vis\
    \ enum $name {}\n        impl $crate::Act for $name {\n            type Operand\
    \ = $operand;\n            type Operator = $operator;\n            fn act(\n \
    \               x: &<Self::Operand as $crate::Algebraic>::Value,\n           \
    \     f: &<Self::Operator as $crate::Algebraic>::Value,\n            ) -> <Self::Operand\
    \ as $crate::Algebraic>::Value {\n                $act(x, f)\n            }\n\
    \        }\n    };\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  isVerificationFile: false
  path: crates/algebra/algebraic_traits/src/macros.rs
  requiredBy:
  - crates/linear_algebra/matrix/src/lib.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/string/rolling_hash/src/lib.rs
  - crates/data_structure/fenwick_tree/fenwick_tree/src/lib.rs
  - crates/data_structure/union_find/union_find/src/lib.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/disjoint_sparse_table/src/lib.rs
  - crates/data_structure/foldable_deque/src/lib.rs
  - crates/data_structure/segment_tree/lazy_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/dual_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/sparse_segment_tree/src/lib.rs
  - crates/data_structure/segment_tree/persistent_segment_tree/src/lib.rs
  - crates/data_structure/wavelet_matrix/src/lib.rs
  - crates/convolution/bitwise_and_convolution/src/lib.rs
  - crates/convolution/lcm_convolution/src/lib.rs
  - crates/convolution/bitwise_or_convolution/src/lib.rs
  - crates/convolution/gcd_convolution/src/lib.rs
  - crates/convolution/bitwise_xor_convolution/src/lib.rs
  - crates/convolution/convolution/src/lib.rs
  - verify/sandbox/test/src/main.rs
  timestamp: '2025-04-22 06:09:15+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  - verify/library_checker/data_structure/unionfind_with_potential_non_commutative_group/src/main.rs
documentation_of: crates/algebra/algebraic_traits/src/macros.rs
layout: document
redirect_from:
- /library/crates/algebra/algebraic_traits/src/macros.rs
- /library/crates/algebra/algebraic_traits/src/macros.rs.html
title: crates/algebra/algebraic_traits/src/macros.rs
---
