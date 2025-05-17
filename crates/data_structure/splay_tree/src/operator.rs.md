---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
    title: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
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
  code: "use algebraic_traits::{Algebraic, Magma, Monoid, Unital};\nuse std::marker::PhantomData;\n\
    \npub struct FoldAct<A>(PhantomData<fn() -> A>);\npub struct Fold<M>(PhantomData<fn()\
    \ -> M>);\npub struct Act<A>(PhantomData<fn() -> A>);\npub struct Noop<T>(PhantomData<fn()\
    \ -> T>);\n\n#[doc(hidden)]\npub trait Operator {\n    type X: Clone;\n    type\
    \ P: Clone;\n    type F: Clone + PartialEq;\n\n    fn single(x: &Self::X) -> Self::P;\n\
    \    fn op(x: &Self::P, y: &Self::P) -> Self::P;\n    fn unit() -> Self::P;\n\
    \    fn unit_act() -> Self::F;\n    fn act_to_val(x: &Self::X, f: &Self::F) ->\
    \ Self::X;\n    fn act_to_prod(x: &Self::P, f: &Self::F) -> Self::P;\n    fn compose(f:\
    \ &Self::F, g: &Self::F) -> Self::F;\n}\n\n#[doc(hidden)]\npub trait Foldable\
    \ {}\nimpl<A> Foldable for FoldAct<A> {}\nimpl<M> Foldable for Fold<M> {}\n\n\
    #[doc(hidden)]\npub trait Actable {}\nimpl<A> Actable for FoldAct<A> {}\nimpl<A>\
    \ Actable for Act<A> {}\n\nimpl<A> Operator for FoldAct<A>\nwhere\n    A: algebraic_traits::Act,\n\
    \    A::Operand: Monoid,\n    <A::Operand as Algebraic>::Value: Clone,\n    A::Operator:\
    \ Monoid,\n    <A::Operator as Algebraic>::Value: Clone + PartialEq,\n{\n    type\
    \ X = <A::Operand as Algebraic>::Value;\n    type P = <A::Operand as Algebraic>::Value;\n\
    \    type F = <A::Operator as Algebraic>::Value;\n\n    fn single(x: &Self::X)\
    \ -> Self::P {\n        x.clone()\n    }\n\n    fn op(x: &Self::P, y: &Self::P)\
    \ -> Self::P {\n        A::Operand::op(x, y)\n    }\n\n    fn unit() -> Self::P\
    \ {\n        A::Operand::unit()\n    }\n\n    fn unit_act() -> Self::F {\n   \
    \     A::Operator::unit()\n    }\n\n    fn act_to_val(x: &Self::X, f: &Self::F)\
    \ -> Self::X {\n        A::act(x, f)\n    }\n\n    fn act_to_prod(x: &Self::P,\
    \ f: &Self::F) -> Self::P {\n        A::act(x, f)\n    }\n\n    fn compose(f:\
    \ &Self::F, g: &Self::F) -> Self::F {\n        A::Operator::op(f, g)\n    }\n\
    }\n\nimpl<M> Operator for Fold<M>\nwhere\n    M: Monoid,\n    <M as Algebraic>::Value:\
    \ Clone,\n{\n    type X = <M as Algebraic>::Value;\n    type P = <M as Algebraic>::Value;\n\
    \    type F = ();\n\n    fn single(x: &Self::X) -> Self::P {\n        x.clone()\n\
    \    }\n\n    fn op(x: &Self::P, y: &Self::P) -> Self::P {\n        M::op(x, y)\n\
    \    }\n\n    fn unit() -> Self::P {\n        M::unit()\n    }\n\n    fn unit_act()\
    \ -> Self::F {}\n\n    fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {\n\
    \        x.clone()\n    }\n\n    fn act_to_prod(x: &Self::P, _: &Self::F) -> Self::P\
    \ {\n        x.clone()\n    }\n\n    fn compose(_: &Self::F, _: &Self::F) -> Self::F\
    \ {}\n}\n\nimpl<A> Operator for Act<A>\nwhere\n    A: algebraic_traits::Act,\n\
    \    A::Operator: Monoid,\n    <A::Operator as Algebraic>::Value: Clone + PartialEq,\n\
    \    <A::Operand as Algebraic>::Value: Clone,\n{\n    type X = <A::Operand as\
    \ Algebraic>::Value;\n    type P = ();\n    type F = <A::Operator as Algebraic>::Value;\n\
    \n    fn single(_: &Self::X) -> Self::P {}\n\n    fn op(_: &Self::P, _: &Self::P)\
    \ -> Self::P {}\n\n    fn unit() -> Self::P {}\n\n    fn unit_act() -> Self::F\
    \ {\n        A::Operator::unit()\n    }\n\n    fn act_to_val(x: &Self::X, f: &Self::F)\
    \ -> Self::X {\n        A::act(x, f)\n    }\n\n    fn act_to_prod(_: &Self::P,\
    \ _: &Self::F) -> Self::P {}\n\n    fn compose(f: &Self::F, g: &Self::F) -> Self::F\
    \ {\n        A::Operator::op(f, g)\n    }\n}\n\nimpl<T> Operator for Noop<T>\n\
    where\n    T: Clone,\n{\n    type X = T;\n    type P = ();\n    type F = ();\n\
    \n    fn single(_: &Self::X) -> Self::P {}\n\n    fn op(_: &Self::P, _: &Self::P)\
    \ -> Self::P {}\n\n    fn unit() -> Self::P {}\n\n    fn unit_act() -> Self::F\
    \ {}\n\n    fn act_to_val(x: &Self::X, _: &Self::F) -> Self::X {\n        x.clone()\n\
    \    }\n\n    fn act_to_prod(_: &Self::P, _: &Self::F) -> Self::P {}\n\n    fn\
    \ compose(_: &Self::F, _: &Self::F) -> Self::F {}\n}\n"
  dependsOn:
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/tree.rs
  isVerificationFile: false
  path: crates/data_structure/splay_tree/src/operator.rs
  requiredBy:
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/data_structure/splay_tree/src/allocator.rs
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
documentation_of: crates/data_structure/splay_tree/src/operator.rs
layout: document
redirect_from:
- /library/crates/data_structure/splay_tree/src/operator.rs
- /library/crates/data_structure/splay_tree/src/operator.rs.html
title: crates/data_structure/splay_tree/src/operator.rs
---
