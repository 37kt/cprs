---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/misc/bit_subset/src/lib.rs
    title: crates/misc/bit_subset/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/chminmax.rs
    title: crates/misc/macros/src/chminmax.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/733
    links:
    - https://yukicoder.me/problems/no/733
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/733\n\n\
    use bit_subset::BitSubsetExt;\nuse macros::chmin;\nuse proconio::{fastout, input};\n\
    \nconst INF: usize = 1 << 60;\n\n#[fastout]\nfn main() {\n    input! {\n     \
    \   m: usize,\n        n: usize,\n        a: [usize; n],\n    }\n    let mut b\
    \ = vec![0; 1 << n];\n    for i in 0..n {\n        for s in 0..1 << n {\n    \
    \        if s >> i & 1 == 0 {\n                b[s | 1 << i] = b[s] + a[i];\n\
    \            }\n        }\n    }\n\n    let mut dp = vec![INF; 1 << n];\n    dp[0]\
    \ = 0;\n    for s in 0usize..1 << n {\n        for t in s.subsets() {\n      \
    \      if b[t] <= m {\n                chmin!(dp[s], dp[s ^ t] + 1);\n       \
    \     }\n        }\n    }\n\n    println!(\"{}\", dp.last().unwrap());\n}\n"
  dependsOn:
  - crates/misc/bit_subset/src/lib.rs
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/mvec.rs
  - crates/misc/macros/src/yes.rs
  isVerificationFile: true
  path: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
  requiredBy: []
  timestamp: '2025-03-23 08:46:19+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
- /verify/verify/yukicoder/dp/yuki0733_bitdp/src/main.rs.html
title: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
---
