---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/algorithm/mo/src/lib.rs
    title: crates/algorithm/mo/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_sum
    links:
    - https://judge.yosupo.jp/problem/static_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_sum\n\
    \nuse itertools::Itertools;\nuse mo::Mo;\nuse proconio::input;\n\nstruct Solver\
    \ {\n    a: Vec<i64>,\n    sum: i64,\n}\n\nimpl Mo for Solver {\n    type Output\
    \ = i64;\n    fn add(&mut self, i: usize) {\n        self.sum += self.a[i];\n\
    \    }\n    fn remove(&mut self, i: usize) {\n        self.sum -= self.a[i];\n\
    \    }\n    fn query(&self) -> Self::Output {\n        self.sum\n    }\n}\n\n\
    #[proconio::fastout]\nfn main() {\n    input! {\n        n: usize,\n        q:\
    \ usize,\n        a: [i64; n],\n        lr: [(usize, usize); q],\n    }\n    let\
    \ mut solver = Solver { a, sum: 0 };\n    println!(\"{}\", solver.solve(&lr).iter().join(\"\
    \\n\"));\n}\n"
  dependsOn:
  - crates/algorithm/mo/src/lib.rs
  isVerificationFile: true
  path: verify/static_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 15:23:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/static_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/static_range_sum/src/main.rs
- /verify/verify/static_range_sum/src/main.rs.html
title: verify/static_range_sum/src/main.rs
---