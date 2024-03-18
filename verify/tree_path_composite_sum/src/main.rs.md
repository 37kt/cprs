---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':question:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':question:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':x:'
    path: crates/tree/re-rooting-dp/src/lib.rs
    title: crates/tree/re-rooting-dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/tree_path_composite_sum
    links:
    - https://judge.yosupo.jp/problem/tree_path_composite_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_path_composite_sum\n\
    \nuse algebraic::{act, algebra, monoid};\nuse graph::Graph;\nuse itertools::Itertools;\n\
    use modint::ModInt998244353 as Mint;\nuse proconio::input;\nuse re_rooting_dp::ReRootingDP;\n\
    \nalgebra!(M, (Mint, Mint));\nmonoid!(M, (0.into(), 0.into()), |&(c1, s1), &(c2,\
    \ s2)| (\n    c1 + c2,\n    s1 + s2\n));\n\nalgebra!(V, Mint);\nact!(V, (Mint,\
    \ Mint), |&v, &(c, s)| (c + 1, s + v));\n\nalgebra!(E, (Mint, Mint));\nact!(E,\
    \ (Mint, Mint), |&(a, b), &(c, s)| (c, a * s + b * c));\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        a: [Mint; n],\n    }\n \
    \   let mut g = Graph::from(a);\n    for _ in 0..n - 1 {\n        input! {\n \
    \           u: usize,\n            v: usize,\n            w: (Mint, Mint),\n \
    \       }\n        g.add_undirected_edge(u, v, w);\n    }\n    let dp = ReRootingDP::build::<M,\
    \ V, E>(&g);\n    println!(\"{}\", (0..n).map(|v| dp.prod(v).1).join(\" \"));\n\
    }\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/graph/graph/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  - crates/tree/re-rooting-dp/src/lib.rs
  isVerificationFile: true
  path: verify/tree_path_composite_sum/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/tree_path_composite_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/tree_path_composite_sum/src/main.rs
- /verify/verify/tree_path_composite_sum/src/main.rs.html
title: verify/tree_path_composite_sum/src/main.rs
---