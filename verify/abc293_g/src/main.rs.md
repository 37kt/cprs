---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/algorithm/mo/src/lib.rs
    title: crates/algorithm/mo/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://atcoder.jp/contests/abc293/tasks/abc293_g
    links:
    - https://atcoder.jp/contests/abc293/tasks/abc293_g
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://atcoder.jp/contests/abc293/tasks/abc293_g\n\
    \nuse itertools::Itertools;\nuse mo::Mo;\nuse proconio::{input, marker::Usize1};\n\
    \nfn f(x: usize) -> usize {\n    if x < 2 {\n        0\n    } else {\n       \
    \ x * (x - 1) * (x - 2) / 6\n    }\n}\n\nstruct Solver {\n    a: Vec<usize>,\n\
    \    cnt: Vec<usize>,\n    sum: usize,\n}\n\nimpl Mo for Solver {\n    type Output\
    \ = usize;\n    fn add(&mut self, i: usize) {\n        self.sum -= f(self.cnt[self.a[i]]);\n\
    \        self.cnt[self.a[i]] += 1;\n        self.sum += f(self.cnt[self.a[i]]);\n\
    \    }\n    fn remove(&mut self, i: usize) {\n        self.sum -= f(self.cnt[self.a[i]]);\n\
    \        self.cnt[self.a[i]] -= 1;\n        self.sum += f(self.cnt[self.a[i]]);\n\
    \    }\n    fn query(&self) -> Self::Output {\n        self.sum\n    }\n}\n\n\
    #[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n        q:\
    \ usize,\n        a: [Usize1; n],\n        lr: [(Usize1, usize); q],\n    }\n\
    \    let mut solver = Solver {\n        a,\n        cnt: vec![0; 200_000],\n \
    \       sum: 0,\n    };\n    println!(\"{}\", solver.solve(&lr).iter().join(\"\
    \\n\"));\n}\n"
  dependsOn:
  - crates/algorithm/mo/src/lib.rs
  isVerificationFile: true
  path: verify/abc293_g/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 14:25:00+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/abc293_g/src/main.rs
layout: document
redirect_from:
- /verify/verify/abc293_g/src/main.rs
- /verify/verify/abc293_g/src/main.rs.html
title: verify/abc293_g/src/main.rs
---
