---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/link-cut-tree/src/lib.rs
    title: crates/data-structure/link-cut-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/dynamic_tree_vertex_set_path_composite
    links:
    - https://judge.yosupo.jp/problem/dynamic_tree_vertex_set_path_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/dynamic_tree_vertex_set_path_composite\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse algebraic::{act, algebra, monoid};\n\
    use link_cut_tree::LinkCutTree;\nuse proconio::input;\n\nalgebra!(M, (Mint, Mint));\n\
    monoid!(M, (1.into(), 0.into()), |&(a, b), &(c, d)| (\n    a * c,\n    a * d +\
    \ b\n));\n\nalgebra!(F, ());\nact!(F, (Mint, Mint), |_, &a| a);\nmonoid!(F, (),\
    \ |_, _| ());\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n        a: [(Mint, Mint); n],\n    }\n    let mut lct = LinkCutTree::<M,\
    \ F>::from(&a[..]);\n    for _ in 0..n - 1 {\n        input! {\n            u:\
    \ usize,\n            v: usize,\n        }\n        lct.link(u, v);\n    }\n \
    \   for _ in 0..q {\n        input! {\n            ty: usize,\n        }\n   \
    \     if ty == 0 {\n            input! {\n                u: usize,\n        \
    \        v: usize,\n                w: usize,\n                x: usize,\n   \
    \         }\n            lct.cut(u, v);\n            lct.link(w, x);\n       \
    \ } else if ty == 1 {\n            input! {\n                p: usize,\n     \
    \           c: (Mint, Mint),\n            }\n            lct.set(p, c);\n    \
    \    } else {\n            input! {\n                u: usize,\n             \
    \   v: usize,\n                x: Mint,\n            }\n            let (a, b)\
    \ = lct.prod(v, u);\n            println!(\"{}\", a * x + b);\n        }\n   \
    \ }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/link-cut-tree/src/lib.rs
  isVerificationFile: true
  path: verify/dynamic_tree_vertex_set_path_composite/src/main.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/dynamic_tree_vertex_set_path_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/dynamic_tree_vertex_set_path_composite/src/main.rs
- /verify/verify/dynamic_tree_vertex_set_path_composite/src/main.rs.html
title: verify/dynamic_tree_vertex_set_path_composite/src/main.rs
---