---
data:
  _extendedDependsOn:
  - icon: ':x:'
    path: crates/data-structure/segtree-beats/src/lib.rs
    title: crates/data-structure/segtree-beats/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum
    links:
    - https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/range_chmin_chmax_add_range_sum\n\
    \nuse proconio::input;\nuse segtree_beats::SegtreeBeats;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        a: [i64;\
    \ n],\n    }\n    let mut seg = SegtreeBeats::from(&a[..]);\n    for _ in 0..q\
    \ {\n        input! {\n            t: usize,\n            l: usize,\n        \
    \    r: usize,\n        }\n        if t == 0 {\n            input! {\n       \
    \         b: i64,\n            }\n            seg.chmin(l..r, b);\n        } else\
    \ if t == 1 {\n            input! {\n                b: i64,\n            }\n\
    \            seg.chmax(l..r, b);\n        } else if t == 2 {\n            input!\
    \ {\n                b: i64,\n            }\n            seg.add(l..r, b);\n \
    \       } else {\n            let s = seg.sum(l..r);\n            println!(\"\
    {}\", s);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/segtree-beats/src/lib.rs
  isVerificationFile: true
  path: verify/range_chmin_chmax_add_range_sum/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 11:20:46+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/range_chmin_chmax_add_range_sum/src/main.rs
layout: document
redirect_from:
- /verify/verify/range_chmin_chmax_add_range_sum/src/main.rs
- /verify/verify/range_chmin_chmax_add_range_sum/src/main.rs.html
title: verify/range_chmin_chmax_add_range_sum/src/main.rs
---
