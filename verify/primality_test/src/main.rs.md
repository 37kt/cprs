---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/primality_test
    links:
    - https://judge.yosupo.jp/problem/primality_test
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test\n\
    \nuse fast_factorize::is_prime;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        q: usize,\n    }\n    for _ in 0..q {\n   \
    \     input! {\n            n: u64,\n        }\n        println!(\"{}\", if is_prime(n)\
    \ { \"Yes\" } else { \"No\" });\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/primality_test/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/primality_test/src/main.rs
layout: document
redirect_from:
- /verify/verify/primality_test/src/main.rs
- /verify/verify/primality_test/src/main.rs.html
title: verify/primality_test/src/main.rs
---
