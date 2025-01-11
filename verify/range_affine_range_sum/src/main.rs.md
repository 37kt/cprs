---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/lazy-segment-tree/src/lib.rs
    title: crates/data-structure/lazy-segment-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_affine_range_sum\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse algebraic::{act, algebra, monoid};\n\
    use lazy_segment_tree::LazySegmentTree;\nuse proconio::input;\n\nalgebra!(M, (Mint,\
    \ Mint));\nmonoid!(M, (0.into(), 0.into()), |&(s1, c1), &(s2, c2)| (\n    s1 +\
    \ s2,\n    c1 + c2\n));\n\nalgebra!(F, (Mint, Mint));\nmonoid!(F, (1.into(), 0.into()),\
    \ |&(a, b), &(c, d)| (\n    a * c,\n    a * d + b\n));\nact!(F, (Mint, Mint),\
    \ |&(a, b), &(s, c)| (a * s + b * c, c));\n\n#[proconio::fastout]\nfn main() {\n\
    \    input! {\n        n: usize,\n        q: usize,\n        a: [Mint; n],\n \
    \   }\n    let a: Vec<_> = a.into_iter().map(|x| (x, 1.into())).collect();\n \
    \   let mut seg = LazySegmentTree::<M, F>::from(a);\n    for _ in 0..q {\n   \
    \     input! {\n            ty: usize,\n            l: usize,\n            r:\
    \ usize,\n        }\n        if ty == 0 {\n            input! {\n            \
    \    b: Mint,\n                c: Mint,\n            }\n            seg.apply_range(l..r,\
    \ (b, c));\n        } else {\n            println!(\"{}\", seg.prod(l..r).0);\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/lazy-segment-tree/src/lib.rs
  isVerificationFile: true
  path: verify/range_affine_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/range_affine_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/range_affine_range_sum/src/main.rs
- /verify/verify/range_affine_range_sum/src/main.rs.html
title: verify/range_affine_range_sum/src/main.rs
---
