---
data:
  _extendedDependsOn:
  - icon: ':question:'
    path: crates/math/fast-factorize/src/lib.rs
    title: crates/math/fast-factorize/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: true
  _pathExtension: rs
  _verificationStatusIcon: ':x:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/primality_test
    links:
    - https://judge.yosupo.jp/problem/primality_test
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/primality_test\n\
    \nuse fast_factorize::is_prime;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        q: usize,\n    }\n    for _ in 0..q {\n   \
    \     input! {\n            n: u64,\n        }\n        println!(\"{}\", if is_prime(n)\
    \ { \"Yes\" } else { \"No\" });\n    }\n}\n"
  dependsOn:
  - crates/math/fast-factorize/src/lib.rs
  isVerificationFile: true
  path: verify/primality_test/src/main.rs
  requiredBy: []
  timestamp: '2023-06-13 17:24:28+09:00'
  verificationStatus: TEST_WRONG_ANSWER
  verifiedWith: []
documentation_of: verify/primality_test/src/main.rs
layout: document
redirect_from:
- /verify/verify/primality_test/src/main.rs
- /verify/verify/primality_test/src/main.rs.html
title: verify/primality_test/src/main.rs
---
