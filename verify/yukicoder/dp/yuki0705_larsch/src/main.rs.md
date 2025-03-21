---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/dp/larsch_simple/src/lib.rs
    title: crates/dp/larsch_simple/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/705
    links:
    - https://yukicoder.me/problems/no/705
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/705\n\n\
    use larsch_simple::larsch;\nuse proconio::{fastout, input};\n\n#[fastout]\nfn\
    \ main() {\n    input! {\n        n: usize,\n        a: [i64; n],\n        x:\
    \ [i64; n],\n        y: [i64; n],\n    }\n    let cube = |x: i64| x.abs().pow(3);\n\
    \    let w = |j, i| cube(a[i - 1] - x[j]) + cube(y[j]);\n    let f = |i, j, &x:\
    \ &i64| x + w(j, i);\n    let min = larsch(n, f, 0);\n    let res = min[n].0;\n\
    \    println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/dp/larsch_simple/src/lib.rs
  isVerificationFile: true
  path: verify/yukicoder/dp/yuki0705_larsch/src/main.rs
  requiredBy: []
  timestamp: '2025-03-20 04:28:46+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/yukicoder/dp/yuki0705_larsch/src/main.rs
layout: document
redirect_from:
- /verify/verify/yukicoder/dp/yuki0705_larsch/src/main.rs
- /verify/verify/yukicoder/dp/yuki0705_larsch/src/main.rs.html
title: verify/yukicoder/dp/yuki0705_larsch/src/main.rs
---
