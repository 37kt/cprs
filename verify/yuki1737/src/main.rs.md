---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/prime-sieve/src/lib.rs
    title: crates/math/prime-sieve/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1737
    links:
    - https://yukicoder.me/problems/no/1737
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1737\n\n\
    use prime_sieve::PrimeSieve;\nuse proconio::input;\n\n#[proconio::fastout]\nfn\
    \ main() {\n    input! {\n        n: usize,\n    }\n    let pr = PrimeSieve::new(n);\n\
    \    let mut res = 0;\n    for (x, k) in pr.factorize(n) {\n        res += x *\
    \ k;\n    }\n    println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/math/prime-sieve/src/lib.rs
  isVerificationFile: true
  path: verify/yuki1737/src/main.rs
  requiredBy: []
  timestamp: '2023-04-21 15:23:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yuki1737/src/main.rs
layout: document
redirect_from:
- /verify/verify/yuki1737/src/main.rs
- /verify/verify/yuki1737/src/main.rs.html
title: verify/yuki1737/src/main.rs
---
