---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/dual-segment-tree/src/lib.rs
    title: crates/data-structure/dual-segment-tree/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_point_get
    links:
    - https://judge.yosupo.jp/problem/range_affine_point_get
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_point_get\n\
    \nuse algebraic::{algebra, monoid};\nuse dual_segment_tree::DualSegmentTree;\n\
    use modint::ModInt998244353 as Mint;\nuse proconio::input;\n\nalgebra!(F, (Mint,\
    \ Mint));\nmonoid!(F, (1.into(), 0.into()), |&(a, b), &(c, d)| (\n    a * c,\n\
    \    a * d + b\n));\n\n#[proconio::fastout]\nfn main() {\n    input! {\n     \
    \   n: usize,\n        q: usize,\n        a: [Mint; n],\n    }\n    let mut seg\
    \ = DualSegmentTree::<F>::new(n);\n    for i in 0..n {\n        seg.apply(i, (0.into(),\
    \ a[i]));\n    }\n    for _ in 0..q {\n        input! {\n            t: usize,\n\
    \        }\n        if t == 0 {\n            input! {\n                l: usize,\n\
    \                r: usize,\n                b: Mint,\n                c: Mint,\n\
    \            }\n            seg.apply_range(l..r, (b, c));\n        } else {\n\
    \            input! {\n                i: usize,\n            }\n            println!(\"\
    {}\", seg.get(i).1);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/dual-segment-tree/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/range_affine_point_get/src/main.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/range_affine_point_get/src/main.rs
layout: document
redirect_from:
- /verify/verify/range_affine_point_get/src/main.rs
- /verify/verify/range_affine_point_get/src/main.rs.html
title: verify/range_affine_point_get/src/main.rs
---
