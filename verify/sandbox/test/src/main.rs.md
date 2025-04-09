---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/add.rs
    title: crates/algebra/algebraic_structure/src/add.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/affine.rs
    title: crates/algebra/algebraic_structure/src/affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/count_sum.rs
    title: crates/algebra/algebraic_structure/src/count_sum.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/countsum_affine.rs
    title: crates/algebra/algebraic_structure/src/countsum_affine.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/lib.rs
    title: crates/algebra/algebraic_structure/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/max.rs
    title: crates/algebra/algebraic_structure/src/max.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/min.rs
    title: crates/algebra/algebraic_structure/src/min.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/mul.rs
    title: crates/algebra/algebraic_structure/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/semiring.rs
    title: crates/algebra/algebraic_structure/src/semiring.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/trivial_group.rs
    title: crates/algebra/algebraic_structure/src/trivial_group.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/algebraic_structure/src/xor.rs
    title: crates/algebra/algebraic_structure/src/xor.rs
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
    path: crates/misc/macros/src/chminmax.rs
    title: crates/misc/macros/src/chminmax.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  - icon: ':warning:'
    path: crates/misc/random/src/lib.rs
    title: crates/misc/random/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/lib.rs
    title: crates/number_theory/modint/modint_61/src/lib.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/numeric.rs
    title: crates/number_theory/modint/modint_61/src/numeric.rs
  - icon: ':warning:'
    path: crates/number_theory/modint/modint_61/src/ops.rs
    title: crates/number_theory/modint/modint_61/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic_structure::act::CountsumAffineOperator;\n\nmod sequence_operator\
    \ {\n    use algebraic_traits::{Algebraic, Magma, Monoid, Unital};\n    use std::marker::PhantomData;\n\
    \n    #[doc(hidden)]\n    pub trait Operator {\n        type X;\n        type\
    \ P;\n        type F;\n\n        fn single(x: &Self::X) -> Self::P;\n        fn\
    \ op(x: &Self::P, y: &Self::P) -> Self::P;\n        fn unit() -> Self::P;\n  \
    \      fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X;\n        fn act_to_prod(x:\
    \ &Self::P, f: &Self::F) -> Self::P;\n    }\n\n    trait Foldable {}\n    impl<A>\
    \ Foldable for FoldAct<A> {}\n    impl<M> Foldable for Fold<M> {}\n\n    trait\
    \ Actable {}\n    impl<A> Actable for FoldAct<A> {}\n    impl<A> Actable for Act<A>\
    \ {}\n\n    pub struct FoldAct<A>(PhantomData<fn() -> A>);\n    pub struct Fold<M>(PhantomData<fn()\
    \ -> M>);\n    pub struct Act<A>(PhantomData<fn() -> A>);\n    pub struct Noop<T>(PhantomData<fn()\
    \ -> T>);\n\n    impl<A> Operator for FoldAct<A>\n    where\n        A: algebraic_traits::Act,\n\
    \        A::Operand: Monoid,\n        <A::Operand as Algebraic>::Value: Clone,\n\
    \    {\n        type X = <A::Operand as Algebraic>::Value;\n        type P = <A::Operand\
    \ as Algebraic>::Value;\n        type F = <A::Operator as Algebraic>::Value;\n\
    \n        fn single(x: &Self::X) -> Self::P {\n            x.clone()\n       \
    \ }\n\n        fn op(x: &Self::P, y: &Self::P) -> Self::P {\n            A::Operand::op(x,\
    \ y)\n        }\n\n        fn unit() -> Self::P {\n            A::Operand::unit()\n\
    \        }\n\n        fn act_to_val(x: &Self::X, f: &Self::F) -> Self::X {\n \
    \           A::act(x, f)\n        }\n\n        fn act_to_prod(x: &Self::P, f:\
    \ &Self::F) -> Self::P {\n            A::act(x, f)\n        }\n    }\n\n    impl<M>\
    \ Operator for Fold<M>\n    where\n        M: Monoid,\n        <M as Algebraic>::Value:\
    \ Clone,\n    {\n        type X = <M as Algebraic>::Value;\n        type P = <M\
    \ as Algebraic>::Value;\n        type F = ();\n\n        fn single(x: &Self::X)\
    \ -> Self::P {\n            x.clone()\n        }\n\n        fn op(x: &Self::P,\
    \ y: &Self::P) -> Self::P {\n            M::op(x, y)\n        }\n\n        fn\
    \ unit() -> Self::P {\n            M::unit()\n        }\n\n        fn act_to_val(x:\
    \ &Self::X, _: &Self::F) -> Self::X {\n            x.clone()\n        }\n\n  \
    \      fn act_to_prod(x: &Self::P, _: &Self::F) -> Self::P {\n            x.clone()\n\
    \        }\n    }\n\n    impl<A> Operator for Act<A>\n    where\n        A: algebraic_traits::Act,\n\
    \    {\n        type X = <A::Operand as Algebraic>::Value;\n        type P = ();\n\
    \        type F = <A::Operator as Algebraic>::Value;\n\n        fn single(_: &Self::X)\
    \ -> Self::P {}\n\n        fn op(_: &Self::P, _: &Self::P) -> Self::P {}\n\n \
    \       fn unit() -> Self::P {}\n\n        fn act_to_val(x: &Self::X, f: &Self::F)\
    \ -> Self::X {\n            A::act(x, f)\n        }\n\n        fn act_to_prod(_:\
    \ &Self::P, _: &Self::F) -> Self::P {}\n    }\n\n    impl<T> Operator for Noop<T>\n\
    \    where\n        T: Clone,\n    {\n        type X = T;\n        type P = ();\n\
    \        type F = ();\n\n        fn single(_: &Self::X) -> Self::P {}\n\n    \
    \    fn op(_: &Self::P, _: &Self::P) -> Self::P {}\n\n        fn unit() -> Self::P\
    \ {}\n\n        fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {\n       \
    \     x.clone()\n        }\n\n        fn act_to_prod(_: &Self::P, _: &Self::F)\
    \ -> Self::P {}\n    }\n\n    pub struct SplayTreeNode<O>\n    where\n       \
    \ O: Operator,\n    {\n        val: O::X,\n        prod: O::P,\n        prod_rev:\
    \ O::P,\n        act: O::F,\n        ch: [Option<Box<SplayTreeNode<O>>>; 2],\n\
    \        len: usize,\n        rev: bool,\n    }\n\n    pub struct SplayTree<O>\n\
    \    where\n        O: Operator,\n    {\n        root: Option<Box<SplayTreeNode<O>>>,\n\
    \    }\n\n    impl<O> Default for SplayTree<O>\n    where\n        O: Operator,\n\
    \    {\n        fn default() -> Self {\n            Self { root: None }\n    \
    \    }\n    }\n}\n\nfn main() {\n    let size = std::mem::size_of::<\n       \
    \ sequence_operator::SplayTreeNode<sequence_operator::FoldAct<CountsumAffineOperator<i32>>>,\n\
    \    >();\n    eprintln!(\"{}\", size);\n\n    let seq = sequence_operator::SplayTree::<sequence_operator::Noop<i32>>::default();\n\
    }\n"
  dependsOn:
  - crates/algebra/algebraic_structure/src/add.rs
  - crates/algebra/algebraic_structure/src/affine.rs
  - crates/algebra/algebraic_structure/src/count_sum.rs
  - crates/algebra/algebraic_structure/src/countsum_affine.rs
  - crates/algebra/algebraic_structure/src/lib.rs
  - crates/algebra/algebraic_structure/src/max.rs
  - crates/algebra/algebraic_structure/src/min.rs
  - crates/algebra/algebraic_structure/src/mul.rs
  - crates/algebra/algebraic_structure/src/semiring.rs
  - crates/algebra/algebraic_structure/src/trivial_group.rs
  - crates/algebra/algebraic_structure/src/xor.rs
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/mvec.rs
  - crates/misc/macros/src/yes.rs
  - crates/misc/random/src/lib.rs
  - crates/number_theory/modint/modint_61/src/lib.rs
  - crates/number_theory/modint/modint_61/src/numeric.rs
  - crates/number_theory/modint/modint_61/src/ops.rs
  isVerificationFile: false
  path: verify/sandbox/test/src/main.rs
  requiredBy: []
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/test/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/test/src/main.rs
- /library/verify/sandbox/test/src/main.rs.html
title: verify/sandbox/test/src/main.rs
---
