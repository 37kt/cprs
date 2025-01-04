---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/splay-tree/src/lib.rs
    title: crates/data-structure/splay-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum
    links:
    - https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse algebraic::{act, algebra, monoid};\n\
    use proconio::input;\nuse splay_tree::SplayTree;\n\nalgebra!(M, (Mint, Mint));\n\
    monoid!(M, (0.into(), 0.into()), |&(s1, c1), &(s2, c2)| (\n    s1 + s2,\n    c1\
    \ + c2\n));\n\nalgebra!(F, (Mint, Mint));\nact!(F, (Mint, Mint), |&(a, b), &(s,\
    \ c)| (a * s + b * c, c));\nmonoid!(F, (1.into(), 0.into()), |&(a, b), &(c, d)|\
    \ (\n    a * c,\n    a * d + b\n));\n\n#[proconio::fastout]\nfn main() {\n   \
    \ input! {\n        n: usize,\n        q: usize,\n        a: [Mint; n],\n    }\n\
    \    let a = a\n        .into_iter()\n        .map(|x| (x, 1.into()))\n      \
    \  .collect::<Vec<(_, _)>>();\n    let mut sp = SplayTree::<M, F>::from(&a[..]);\n\
    \    for _ in 0..q {\n        input! {\n            ty: usize,\n        }\n  \
    \      if ty == 0 {\n            input! {\n                i: usize,\n       \
    \         x: Mint,\n            }\n            sp.insert(i, (x, 1.into()));\n\
    \        } else if ty == 1 {\n            input! {\n                i: usize,\n\
    \            }\n            sp.remove(i);\n        } else if ty == 2 {\n     \
    \       input! {\n                l: usize,\n                r: usize,\n     \
    \       }\n            sp.reverse(l..r);\n        } else if ty == 3 {\n      \
    \      input! {\n                l: usize,\n                r: usize,\n      \
    \          b: Mint,\n                c: Mint,\n            }\n            sp.apply(l..r,\
    \ (b, c));\n        } else {\n            input! {\n                l: usize,\n\
    \                r: usize,\n            }\n            let (res, _) = sp.prod(l..r);\n\
    \            println!(\"{}\", res);\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/splay-tree/src/lib.rs
  isVerificationFile: true
  path: verify/dynamic_sequence_range_affine_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 02:49:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/dynamic_sequence_range_affine_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/dynamic_sequence_range_affine_range_sum/src/main.rs
- /verify/verify/dynamic_sequence_range_affine_range_sum/src/main.rs.html
title: verify/dynamic_sequence_range_affine_range_sum/src/main.rs
---
