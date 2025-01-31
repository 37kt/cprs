---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algorithm/monotone-minima/src/lib.rs
    title: crates/algorithm/monotone-minima/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/952
    links:
    - https://yukicoder.me/problems/no/952
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/952\n\n\
    use monotone_minima::monotone_minima;\nuse proconio::input;\n\nconst INF: usize\
    \ = 1 << 60;\n\nfn main() {\n    input! {\n        n: usize,\n        a: [usize;\
    \ n],\n    }\n    let mut b = vec![0; n + 1];\n    for i in 0..n {\n        b[i\
    \ + 1] = b[i] + a[i];\n    }\n    let mut dp = vec![INF; n + 2];\n    dp[0] =\
    \ 0;\n    let mut res = vec![0; n];\n    for k in 1..=n {\n        let f = |i:\
    \ usize, j: usize| {\n            if i > j {\n                dp[j] + (b[i - 1]\
    \ - b[j]).pow(2)\n            } else {\n                INF\n            }\n \
    \       };\n        let select = |i, j, k| f(i, j) > f(i, k);\n        let mn\
    \ = monotone_minima(n + 2, n + 2, select);\n        let mut dpn = vec![INF; n\
    \ + 2];\n        for i in k..=n + 1 {\n            dpn[i] = f(i, mn[i]);\n   \
    \     }\n        dp = dpn;\n        res[n - k] = dp[n + 1];\n    }\n    for i\
    \ in 0..n {\n        println!(\"{}\", res[i]);\n    }\n}\n"
  dependsOn:
  - crates/algorithm/monotone-minima/src/lib.rs
  isVerificationFile: true
  path: verify/yuki952/src/main.rs
  requiredBy: []
  timestamp: '2024-12-25 03:34:39+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yuki952/src/main.rs
layout: document
redirect_from:
- /verify/verify/yuki952/src/main.rs
- /verify/verify/yuki952/src/main.rs.html
title: verify/yuki952/src/main.rs
---
