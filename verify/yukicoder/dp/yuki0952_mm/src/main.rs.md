---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/dp/monotone_minima/src/lib.rs
    title: crates/dp/monotone_minima/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/952
    links:
    - https://yukicoder.me/problems/no/952
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/952\n\n\
    use monotone_minima::monotone_minima;\nuse proconio::{fastout, input};\n\nconst\
    \ INF: i64 = 1 << 60;\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        a: [i64; n],\n    }\n    let mut b = vec![0; n + 1];\n    for i in 0..n\
    \ {\n        b[i + 1] = b[i] + a[i];\n    }\n\n    let mut dp = vec![INF; n +\
    \ 2];\n    dp[0] = 0;\n\n    // \u30EB\u30FC\u30D7 1 \u56DE\u3054\u3068\u306B\u958B\
    \u3051\u306A\u3044\u30C9\u30A2\u3092 1 \u3064\u9078\u3076\n    let mut res = vec![INF;\
    \ n + 1];\n    for k in 1..=n {\n        let f = |i: usize, j: usize| {\n    \
    \        if i > j {\n                dp[j] + (b[i - 1] - b[j]).pow(2)\n      \
    \      } else {\n                INF\n            }\n        };\n        let select\
    \ = |i, j, k| f(i, j) > f(i, k);\n        let argmin = monotone_minima(n + 2,\
    \ n + 2, select);\n        dp = (0..n + 2).map(|i| f(i, argmin[i])).collect();\n\
    \        res[n + 1 - k] = dp[n + 1];\n    }\n\n    for i in 1..=n {\n        println!(\"\
    {}\", res[i]);\n    }\n}\n"
  dependsOn:
  - crates/dp/monotone_minima/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/dp/yuki0952_mm/src/main.rs
  requiredBy: []
  timestamp: '2025-03-20 00:46:18+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/dp/yuki0952_mm/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/dp/yuki0952_mm/src/main.rs
- /verify/verify/yukicoder/dp/yuki0952_mm/src/main.rs.html
title: verify/yukicoder/dp/yuki0952_mm/src/main.rs
---
