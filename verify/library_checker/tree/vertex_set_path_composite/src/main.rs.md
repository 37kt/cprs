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
    path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
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
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/compress.rs
    title: crates/tree/heavy_light_decomposition/src/compress.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/construct.rs
    title: crates/tree/heavy_light_decomposition/src/construct.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/heavy_light_decomposition/src/lib.rs
    title: crates/tree/heavy_light_decomposition/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite\n\
    \nuse algebraic_structure::magma::{Affine, AffineOperator};\nuse algebraic_traits::{Magma,\
    \ Unital};\nuse heavy_light_decomposition::HeavyLightDecomposition;\nuse proconio::{fastout,\
    \ input};\nuse segment_tree::SegmentTree;\nuse static_modint::ModInt998244353\
    \ as Mint;\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n     \
    \   q: usize,\n        ab: [(Mint, Mint); n],\n        uv: [(usize, usize); n\
    \ - 1],\n    }\n    let (a, b): (Vec<_>, Vec<_>) = ab.into_iter().unzip();\n \
    \   let hld = HeavyLightDecomposition::from_edges(&uv, 0);\n    let tour = hld.euler_tour().collect::<Vec<_>>();\n\
    \    let mut seg = SegmentTree::<AffineOperator<_>>::from_fn(n, |i| Affine(a[tour[i]],\
    \ b[tour[i]]));\n    let mut ges = SegmentTree::<AffineOperator<_>>::from_fn(n,\
    \ |i| {\n        Affine(a[tour[n - 1 - i]], b[tour[n - 1 - i]])\n    });\n   \
    \ for _ in 0..q {\n        input! {\n            t: usize,\n        }\n      \
    \  if t == 0 {\n            input! {\n                p: usize,\n            \
    \    a: Mint,\n                b: Mint,\n            }\n            let i = hld.vertex_index(p);\n\
    \            seg.set(i, Affine(a, b));\n            ges.set(n - 1 - i, Affine(a,\
    \ b));\n        } else {\n            input! {\n                s: usize,\n  \
    \              t: usize,\n                x: Mint,\n            }\n          \
    \  let mut f = AffineOperator::unit();\n            hld.path_query(s, t, true,\
    \ |l, r, rev| {\n                let g = if rev {\n                    ges.fold(n\
    \ - r..n - l)\n                } else {\n                    seg.fold(l..r)\n\
    \                };\n                f = AffineOperator::op(&f, &g);\n       \
    \     });\n            let res = f.eval(x);\n            println!(\"{}\", res);\n\
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
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  - crates/number_theory/modint/static_modint/src/lib.rs
  - crates/number_theory/modint/static_modint/src/mod_arithmetic.rs
  - crates/number_theory/modint/static_modint/src/ntt_precalc.rs
  - crates/number_theory/modint/static_modint/src/numeric.rs
  - crates/number_theory/modint/static_modint/src/ops.rs
  - crates/tree/heavy_light_decomposition/src/compress.rs
  - crates/tree/heavy_light_decomposition/src/construct.rs
  - crates/tree/heavy_light_decomposition/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-04-07 08:03:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/tree/vertex_set_path_composite/src/main.rs
- /verify/verify/library_checker/tree/vertex_set_path_composite/src/main.rs.html
title: verify/library_checker/tree/vertex_set_path_composite/src/main.rs
---
