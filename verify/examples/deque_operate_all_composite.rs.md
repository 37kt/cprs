---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/deque_operate_all_composite
    links:
    - https://judge.yosupo.jp/problem/deque_operate_all_composite
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite\n\
    \nuse ac_library::ModInt998244353 as Mint;\nuse algebraic::{monoid, Monoid};\n\
    use proconio::input;\nuse sliding_window_aggregation::SlidingWindowAggregation;\n\
    \nmonoid! {\n    M,\n    (Mint, Mint),\n    (1.into(), 0.into()),\n    |&(a, b),\
    \ &(c, d)| (a * c, b * c + d)\n}\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        q: usize,\n    }\n    let mut swag = SlidingWindowAggregation::<M>::new();\n\
    \    for _ in 0..q {\n        input! {\n            t: usize,\n        }\n   \
    \     if t == 0 {\n            input! {\n                f: (Mint, Mint),\n  \
    \          }\n            swag.push_front(f);\n        } else if t == 1 {\n  \
    \          input! {\n                f: (Mint, Mint),\n            }\n       \
    \     swag.push_back(f);\n        } else if t == 2 {\n            swag.pop_front();\n\
    \        } else if t == 3 {\n            swag.pop_back();\n        } else {\n\
    \            input! {\n                x: Mint,\n            }\n            let\
    \ (a, b) = swag.prod();\n            println!(\"{}\", a * x + b);\n        }\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/examples/deque_operate_all_composite.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/examples/deque_operate_all_composite.rs
layout: document
redirect_from:
- /verify/verify/examples/deque_operate_all_composite.rs
- /verify/verify/examples/deque_operate_all_composite.rs.html
title: verify/examples/deque_operate_all_composite.rs
---
