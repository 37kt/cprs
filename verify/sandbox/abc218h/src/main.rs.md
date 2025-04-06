---
data:
  _extendedDependsOn:
  - icon: ':warning:'
    path: crates/dp/aliens_dp/src/lib.rs
    title: crates/dp/aliens_dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use aliens_dp::aliens_dp;\nuse proconio::{fastout, input};\n\nconst INF:\
    \ i64 = 1 << 60;\nconst C: i64 = 1 << 40;\n\n#[fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        r: usize,\n        a: [i64; n - 1],\n    }\n\n\
    \    let g = |p: i64| {\n        let mut dp = [0, p];\n        for &x in &a {\n\
    \            let mut ndp = [INF; 2];\n            #[allow(clippy::needless_range_loop)]\n\
    \            for i in 0..2 {\n                for j in 0..2 {\n              \
    \      let mut cost = 0;\n                    if i != j {\n                  \
    \      cost -= x;\n                    }\n                    if j == 1 {\n  \
    \                      cost += p;\n                    }\n                   \
    \ ndp[j] = ndp[j].min(dp[i] + cost);\n                }\n            }\n     \
    \       dp = ndp;\n        }\n        dp[0].min(dp[1])\n    };\n\n    let res\
    \ = aliens_dp(r, -C..=C, g);\n    println!(\"{}\", -res);\n}\n"
  dependsOn:
  - crates/dp/aliens_dp/src/lib.rs
  isVerificationFile: false
  path: verify/sandbox/abc218h/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/abc218h/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/abc218h/src/main.rs
- /library/verify/sandbox/abc218h/src/main.rs.html
title: verify/sandbox/abc218h/src/main.rs
---
