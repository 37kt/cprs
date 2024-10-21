---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/interval-heap/src/lib.rs
    title: crates/data-structure/interval-heap/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/double_ended_priority_queue
    links:
    - https://judge.yosupo.jp/problem/double_ended_priority_queue
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue\n\
    \nuse interval_heap::IntervalHeap;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        s: [i64;\
    \ n],\n    }\n    let mut pq = IntervalHeap::from(s);\n    for _ in 0..q {\n \
    \       input! {\n            t: usize,\n        }\n        if t == 0 {\n    \
    \        input! {\n                x: i64,\n            }\n            pq.push(x);\n\
    \        } else if t == 1 {\n            println!(\"{}\", pq.pop_min().unwrap());\n\
    \        } else {\n            println!(\"{}\", pq.pop_max().unwrap());\n    \
    \    }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/interval-heap/src/lib.rs
  isVerificationFile: true
  path: verify/double_ended_priority_queue/src/main.rs
  requiredBy: []
  timestamp: '2024-04-08 11:43:23+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/double_ended_priority_queue/src/main.rs
layout: document
redirect_from:
- /verify/verify/double_ended_priority_queue/src/main.rs
- /verify/verify/double_ended_priority_queue/src/main.rs.html
title: verify/double_ended_priority_queue/src/main.rs
---
