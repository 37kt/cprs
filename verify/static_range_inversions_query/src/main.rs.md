---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/mo/src/lib.rs
    title: crates/algorithm/mo/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/fenwick-tree/src/lib.rs
    title: crates/data-structure/fenwick-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_inversions_query
    links:
    - https://judge.yosupo.jp/problem/static_range_inversions_query
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.4/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_inversions_query\n\
    \nuse algebraic::{algebra, group};\nuse fenwick_tree::FenwickTree;\nuse itertools::Itertools;\n\
    use mo::Mo;\nuse proconio::input;\nuse superslice::Ext;\n\nalgebra!(G, i64);\n\
    group!(G, 0, |x, y| x + y, |x: &_| -x);\n\nstruct Solver {\n    a: Vec<usize>,\n\
    \    tr: FenwickTree<G>,\n    res: i64,\n}\n\nimpl Mo for Solver {\n    type Output\
    \ = i64;\n    fn add_left(&mut self, i: usize) {\n        self.res += self.tr.accum(self.a[i]);\n\
    \        self.tr.add(self.a[i], 1);\n    }\n    fn remove_left(&mut self, i: usize)\
    \ {\n        self.res -= self.tr.accum(self.a[i]);\n        self.tr.add(self.a[i],\
    \ -1);\n    }\n    fn add_right(&mut self, i: usize) {\n        self.res += self.tr.sum(self.a[i]\
    \ + 1..);\n        self.tr.add(self.a[i], 1);\n    }\n    fn remove_right(&mut\
    \ self, i: usize) {\n        self.res -= self.tr.sum(self.a[i] + 1..);\n     \
    \   self.tr.add(self.a[i], -1);\n    }\n    fn query(&self) -> Self::Output {\n\
    \        self.res\n    }\n}\n\n#[proconio::fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n        a: [i64; n],\n        lr: [(usize,\
    \ usize); q],\n    }\n    let mut z = a.clone();\n    z.sort();\n    z.dedup();\n\
    \    let mut b = vec![0; n];\n    for i in 0..n {\n        b[i] = z.lower_bound(&a[i]);\n\
    \    }\n    let mut solver = Solver {\n        a: b,\n        tr: FenwickTree::new(z.len()),\n\
    \        res: 0,\n    };\n    let res = solver.solve(&lr);\n    println!(\"{}\"\
    , res.iter().join(\"\\n\"));\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/algorithm/mo/src/lib.rs
  - crates/data-structure/fenwick-tree/src/lib.rs
  isVerificationFile: true
  path: verify/static_range_inversions_query/src/main.rs
  requiredBy: []
  timestamp: '2023-04-25 15:51:20+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/static_range_inversions_query/src/main.rs
layout: document
redirect_from:
- /verify/verify/static_range_inversions_query/src/main.rs
- /verify/verify/static_range_inversions_query/src/main.rs.html
title: verify/static_range_inversions_query/src/main.rs
---
