---
data:
  _extendedDependsOn:
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
    path: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
    title: crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/segment_tree/segment_tree/src/lib.rs
    title: crates/data_structure/segment_tree/segment_tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/majority_voting
    links:
    - https://judge.yosupo.jp/problem/majority_voting
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/majority_voting\n\
    \nuse algebraic_traits::define_algebra;\nuse fenwick_tree_01::FenwickTree01;\n\
    use proconio::fastout;\nuse proconio::input;\nuse segment_tree::SegmentTree;\n\
    \nconst C: usize = 1_000_000_010;\n\ndefine_algebra! {\n    name: S,\n    value:\
    \ (u32, u32),\n    op: |&(m1, c1): &(u32, u32), &(m2, c2): &(u32, u32)| {\n  \
    \      if m1 == m2 {\n            (m1, c1 + c2)\n        } else if c1 >= c2 {\n\
    \            (m1, c1 - c2)\n        } else {\n            (m2, c2 - c1)\n    \
    \    }\n    },\n    unit: (0, 0),\n    associative,\n}\n\n#[fastout]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        q: usize,\n        mut a: [usize;\
    \ n],\n        mut queries: [(usize, usize, usize); q],\n    }\n\n    let mut\
    \ z: Vec<usize> = a.iter().enumerate().map(|(i, &x)| x * C + i).collect();\n \
    \   for &(t, x, y) in &queries {\n        if t == 0 {\n            z.push(y *\
    \ C + x);\n        }\n    }\n    z.sort_unstable();\n    z.dedup();\n\n    let\
    \ idx = |x: usize| z.partition_point(|&y| y < x);\n    let mut seg = SegmentTree::<S>::from_fn(n,\
    \ |i| (a[i] as u32, 1));\n    let mut ft = FenwickTree01::new(z.len());\n    for\
    \ (i, &x) in a.iter().enumerate() {\n        ft.set(idx(x * C + i), 1);\n    }\n\
    \    for &(t, x, y) in &queries {\n        if t == 0 {\n            let i = idx(a[x]\
    \ * C + x);\n            ft.set(i, 0);\n            let i = idx(y * C + x);\n\
    \            ft.set(i, 1);\n            a[x] = y;\n            seg.set(x, (y as\
    \ u32, 1));\n        } else {\n            let m = seg.fold(x..y).0 as usize;\n\
    \            let l = idx(m * C + x);\n            let r = idx(m * C + y);\n  \
    \          let c = ft.fold(l..r);\n            if c * 2 > y - x {\n          \
    \      println!(\"{}\", m);\n            } else {\n                println!(\"\
    -1\");\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/algebra/algebraic_traits/src/lib.rs
  - crates/algebra/algebraic_traits/src/macros.rs
  - crates/algebra/algebraic_traits/src/traits.rs
  - crates/data_structure/fenwick_tree/fenwick_tree_01/src/lib.rs
  - crates/data_structure/segment_tree/segment_tree/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/majority_voting/src/main.rs
  requiredBy: []
  timestamp: '2025-06-07 03:16:25+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/majority_voting/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/majority_voting/src/main.rs
- /verify/verify/library_checker/data_structure/majority_voting/src/main.rs.html
title: verify/library_checker/data_structure/majority_voting/src/main.rs
---
