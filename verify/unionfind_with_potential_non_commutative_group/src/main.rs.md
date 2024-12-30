---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/potentialized-union-find/src/lib.rs
    title: crates/data-structure/potentialized-union-find/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
    links:
    - https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind_with_potential_non_commutative_group\n\
    \nuse std::ops::{Add, Neg};\n\nuse modint::ModInt998244353 as Mint;\nuse potentialized_union_find::PotentializedUnionFind;\n\
    use proconio::input;\n\n#[derive(Clone, Copy, Eq, PartialEq)]\nstruct T([[Mint;\
    \ 2]; 2]);\n\nimpl Add for T {\n    type Output = T;\n    fn add(self, rhs: Self)\
    \ -> Self::Output {\n        T([\n            [\n                self.0[0][0]\
    \ * rhs.0[0][0] + self.0[0][1] * rhs.0[1][0],\n                self.0[0][0] *\
    \ rhs.0[0][1] + self.0[0][1] * rhs.0[1][1],\n            ],\n            [\n \
    \               self.0[1][0] * rhs.0[0][0] + self.0[1][1] * rhs.0[1][0],\n   \
    \             self.0[1][0] * rhs.0[0][1] + self.0[1][1] * rhs.0[1][1],\n     \
    \       ],\n        ])\n    }\n}\n\nimpl Neg for T {\n    type Output = T;\n \
    \   fn neg(self) -> Self::Output {\n        T([[self.0[1][1], -self.0[0][1]],\
    \ [-self.0[1][0], self.0[0][0]]])\n    }\n}\n\nimpl Default for T {\n    fn default()\
    \ -> Self {\n        T([[Mint::new(1), Mint::new(0)], [Mint::new(0), Mint::new(1)]])\n\
    \    }\n}\n\n#[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        q: usize,\n    }\n    let mut uf = PotentializedUnionFind::<T>::new(n);\n\
    \    for _ in 0..q {\n        input! {\n            t: usize,\n        }\n   \
    \     if t == 0 {\n            input! {\n                u: usize,\n         \
    \       v: usize,\n                x: [[Mint; 2]; 2],\n            }\n       \
    \     let valid = uf.merge(u, v, T([[x[0][0], x[0][1]], [x[1][0], x[1][1]]]));\n\
    \            println!(\"{}\", valid as i32);\n        } else {\n            input!\
    \ {\n                u: usize,\n                v: usize,\n            }\n   \
    \         if let Some(T(diff)) = uf.diff(u, v) {\n                println!(\n\
    \                    \"{} {} {} {}\",\n                    diff[0][0], diff[0][1],\
    \ diff[1][0], diff[1][1]\n                );\n            } else {\n         \
    \       println!(\"-1\");\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/potentialized-union-find/src/lib.rs
  - crates/number-theory/modint/src/lib.rs
  isVerificationFile: true
  path: verify/unionfind_with_potential_non_commutative_group/src/main.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/unionfind_with_potential_non_commutative_group/src/main.rs
layout: document
redirect_from:
- /verify/verify/unionfind_with_potential_non_commutative_group/src/main.rs
- /verify/verify/unionfind_with_potential_non_commutative_group/src/main.rs.html
title: verify/unionfind_with_potential_non_commutative_group/src/main.rs
---
