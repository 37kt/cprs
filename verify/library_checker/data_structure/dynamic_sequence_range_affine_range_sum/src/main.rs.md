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
    path: crates/data_structure/splay_tree/src/allocator.rs
    title: crates/data_structure/splay_tree/src/allocator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/lib.rs
    title: crates/data_structure/splay_tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/node.rs
    title: crates/data_structure/splay_tree/src/node.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/operator.rs
    title: crates/data_structure/splay_tree/src/operator.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/sequence.rs
    title: crates/data_structure/splay_tree/src/sequence.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/splay_tree/src/tree.rs
    title: crates/data_structure/splay_tree/src/tree.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/lib.rs
    title: crates/number_theory/modint/static_modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
    title: crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
    title: crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/numeric.rs
    title: crates/number_theory/modint/static_modint/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/number_theory/modint/static_modint/src/ops.rs
    title: crates/number_theory/modint/static_modint/src/ops.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum\n\
    \nuse algebraic_structure::act::CountsumAffineOperator;\nuse algebraic_structure::magma::{Affine,\
    \ CountSum};\nuse proconio::fastout;\nuse proconio::input;\nuse splay_tree::{FoldAct,\
    \ SplaySequence};\nuse static_modint::ModInt998244353 as Mint;\n\n#[fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [Mint;\
    \ n],\n    }\n\n    let mut seq =\n        SplaySequence::<FoldAct<CountsumAffineOperator<Mint>>>::from_fn(n,\
    \ |i| CountSum {\n            count: 1.into(),\n            sum: a[i],\n     \
    \   });\n\n    for _ in 0..q {\n        input! {\n            t: usize,\n    \
    \    }\n\n        match t {\n            0 => {\n                input! {\n  \
    \                  i: usize,\n                    x: Mint,\n                }\n\
    \                seq.insert(\n                    i,\n                    CountSum\
    \ {\n                        count: 1.into(),\n                        sum: x,\n\
    \                    },\n                );\n            }\n            1 => {\n\
    \                input! {\n                    i: usize,\n                }\n\
    \                seq.remove(i);\n            }\n            2 => {\n         \
    \       input! {\n                    l: usize,\n                    r: usize,\n\
    \                }\n                seq.reverse(l..r);\n            }\n      \
    \      3 => {\n                input! {\n                    l: usize,\n     \
    \               r: usize,\n                    a: Mint,\n                    b:\
    \ Mint,\n                }\n                seq.apply(l..r, Affine(a, b));\n \
    \           }\n            4 => {\n                input! {\n                \
    \    l: usize,\n                    r: usize,\n                }\n           \
    \     println!(\"{}\", seq.fold(l..r).sum);\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
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
  - crates/data_structure/splay_tree/src/allocator.rs
  - crates/data_structure/splay_tree/src/lib.rs
  - crates/data_structure/splay_tree/src/node.rs
  - crates/data_structure/splay_tree/src/operator.rs
  - crates/data_structure/splay_tree/src/sequence.rs
  - crates/data_structure/splay_tree/src/tree.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-04-09 07:52:13+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
- /verify/verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs.html
title: verify/library_checker/data_structure/dynamic_sequence_range_affine_range_sum/src/main.rs
---
