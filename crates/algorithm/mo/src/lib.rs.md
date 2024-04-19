---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/static_range_inversions_query/src/main.rs
    title: verify/static_range_inversions_query/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/static_range_sum/src/main.rs
    title: verify/static_range_sum/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.3/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub trait Mo {\n    type Output: Default + Clone;\n\n    #[allow(unused_variables)]\n\
    \    fn add(&mut self, i: usize) {\n        unimplemented!()\n    }\n\n    #[allow(unused_variables)]\n\
    \    fn remove(&mut self, i: usize) {\n        unimplemented!()\n    }\n\n   \
    \ fn add_left(&mut self, i: usize) {\n        self.add(i);\n    }\n\n    fn add_right(&mut\
    \ self, i: usize) {\n        self.add(i);\n    }\n\n    fn remove_left(&mut self,\
    \ i: usize) {\n        self.remove(i);\n    }\n\n    fn remove_right(&mut self,\
    \ i: usize) {\n        self.remove(i);\n    }\n\n    fn query(&self) -> Self::Output;\n\
    \n    fn solve(&mut self, qs: &[(usize, usize)]) -> Vec<Self::Output> {\n    \
    \    let n = qs.iter().map(|&(_, r)| r).max().unwrap();\n        let q = qs.len();\n\
    \        let w = n / n.max(1).min((q.max(1) as f64).sqrt().round() as usize);\n\
    \        let mut ord = (0..q).collect::<Vec<_>>();\n        ord.sort_by_key(|&i|\
    \ {\n            let (l, r) = qs[i];\n            (l / w, if (l / w) & 1 == 0\
    \ { r } else { !r })\n        });\n        let mut l = 0;\n        let mut r =\
    \ 0;\n\n        let mut res = vec![Default::default(); q];\n        for i in ord\
    \ {\n            let (ll, rr) = qs[i];\n            while l > ll {\n         \
    \       l -= 1;\n                self.add_left(l);\n            }\n          \
    \  while r < rr {\n                self.add_right(r);\n                r += 1;\n\
    \            }\n            while l < ll {\n                self.remove_left(l);\n\
    \                l += 1;\n            }\n            while r > rr {\n        \
    \        r -= 1;\n                self.remove_right(r);\n            }\n     \
    \       res[i] = self.query();\n        }\n        res\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/mo/src/lib.rs
  requiredBy: []
  timestamp: '2023-04-20 08:15:08+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/static_range_sum/src/main.rs
  - verify/static_range_inversions_query/src/main.rs
documentation_of: crates/algorithm/mo/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/mo/src/lib.rs
- /library/crates/algorithm/mo/src/lib.rs.html
title: crates/algorithm/mo/src/lib.rs
---
