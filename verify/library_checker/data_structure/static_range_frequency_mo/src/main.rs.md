---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/mo/src/lib.rs
    title: crates/algorithm/mo/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/coordinate_compression/src/lib.rs
    title: crates/misc/coordinate_compression/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/static_range_frequency
    links:
    - https://judge.yosupo.jp/problem/static_range_frequency
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/static_range_frequency\n\
    \nuse coordinate_compression::CoordinateCompression;\nuse mo::Mo;\nuse proconio::fastout;\n\
    use proconio::input;\n\nstruct Solver<'a> {\n    a: &'a [usize],\n    cnt: Vec<usize>,\n\
    }\n\nimpl<'a> Mo for Solver<'a> {\n    type Arg = usize;\n    type Output = usize;\n\
    \n    fn add(&mut self, i: usize) {\n        self.cnt[self.a[i]] += 1;\n    }\n\
    \n    fn remove(&mut self, i: usize) {\n        self.cnt[self.a[i]] -= 1;\n  \
    \  }\n\n    fn query(&mut self, &query: &Self::Arg) -> Self::Output {\n      \
    \  self.cnt[query]\n    }\n}\n\n#[fastout]\nfn main() {\n    input! {\n      \
    \  n: usize,\n        q: usize,\n        mut a: [usize; n],\n        mut lrx:\
    \ [(usize, usize, usize); q],\n    }\n\n    let xs = a\n        .iter()\n    \
    \    .copied()\n        .chain(lrx.iter().map(|&(_, _, x)| x))\n        .collect::<Vec<_>>();\n\
    \    let (cc, xs) = CoordinateCompression::<_>::new(xs);\n    let (a, x) = xs.split_at(n);\n\
    \    for i in 0..q {\n        lrx[i].2 = x[i];\n    }\n\n    let m = cc.len();\n\
    \n    let mut solver = Solver {\n        a: &a,\n        cnt: vec![0; m],\n  \
    \  };\n    let res = solver.solve(&lrx);\n    for x in res {\n        println!(\"\
    {}\", x);\n    }\n}\n"
  dependsOn:
  - crates/algorithm/mo/src/lib.rs
  - crates/misc/coordinate_compression/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
  requiredBy: []
  timestamp: '2025-03-16 01:09:41+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
- /verify/verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs.html
title: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
---
