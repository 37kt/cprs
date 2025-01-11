---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/sliding-window-aggregation/src/lib.rs
    title: crates/data-structure/sliding-window-aggregation/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/deque_operate_all_composite
    links:
    - https://judge.yosupo.jp/problem/deque_operate_all_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse algebraic::{algebra, monoid};\n\
    use proconio::input;\nuse sliding_window_aggregation::SlidingWindowAggregation;\n\
    \nalgebra!(M, (Mint, Mint));\nmonoid!(M, (1.into(), 0.into()), |&(a, b), &(c,\
    \ d)| (\n    a * c,\n    b * c + d\n));\n\n#[proconio::fastout]\nfn main() {\n\
    \    input! {\n        q: usize,\n    }\n    let mut swag = SlidingWindowAggregation::<M>::new();\n\
    \    for _ in 0..q {\n        input! {\n            t: usize,\n        }\n   \
    \     if t == 0 {\n            input! {\n                f: (Mint, Mint),\n  \
    \          }\n            swag.push_front(f);\n        } else if t == 1 {\n  \
    \          input! {\n                f: (Mint, Mint),\n            }\n       \
    \     swag.push_back(f);\n        } else if t == 2 {\n            swag.pop_front();\n\
    \        } else if t == 3 {\n            swag.pop_back();\n        } else {\n\
    \            input! {\n                x: Mint,\n            }\n            let\
    \ (a, b) = swag.prod();\n            println!(\"{}\", a * x + b);\n        }\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/sliding-window-aggregation/src/lib.rs
  isVerificationFile: true
  path: verify/deque_operate_all_composite/src/main.rs
  requiredBy: []
  timestamp: '2025-01-11 09:03:35+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/deque_operate_all_composite/src/main.rs
layout: document
redirect_from:
- /verify/verify/deque_operate_all_composite/src/main.rs
- /verify/verify/deque_operate_all_composite/src/main.rs.html
title: verify/deque_operate_all_composite/src/main.rs
---
