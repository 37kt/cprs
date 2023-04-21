---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/graph/graph/src/lib.rs
    title: crates/graph/graph/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/tree/re-rooting-dp/src/lib.rs
    title: crates/tree/re-rooting-dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1333
    links:
    - https://yukicoder.me/problems/no/1333
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1333\n\n\
    use ac_library::ModInt1000000007 as M;\nuse graph::Graph;\nuse proconio::{input,\
    \ marker::Usize1};\nuse re_rooting_dp::ReRootingDP;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n    }\n    let mut g = Graph::<(),\
    \ M>::new(n);\n    for _ in 0..n - 1 {\n        input! {\n            u: Usize1,\n\
    \            v: Usize1,\n            w: M,\n        }\n        g.add_undirected_edge(u,\
    \ v, w);\n    }\n    let dp = ReRootingDP::build(\n        &g,\n        (M::new(0),\
    \ M::new(0), M::new(0)),\n        |&(s1, sq1, c1), &(s2, sq2, c2)| (s1 + s2, sq1\
    \ + sq2, c1 + c2),\n        |&(s, sq, c), _| (s, sq, c + 1),\n        |&(s, sq,\
    \ c), w| (s + c * w, sq + s * w * 2 + w * w * c, c),\n    );\n    let mut res\
    \ = M::new(0);\n    for i in 0..n {\n        res += dp.prod(i).1;\n    }\n   \
    \ res /= 2;\n    println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/graph/graph/src/lib.rs
  - crates/tree/re-rooting-dp/src/lib.rs
  isVerificationFile: true
  path: verify/yuki1333/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 16:14:58+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yuki1333/src/main.rs
layout: document
redirect_from:
- /verify/verify/yuki1333/src/main.rs
- /verify/verify/yuki1333/src/main.rs.html
title: verify/yuki1333/src/main.rs
---
