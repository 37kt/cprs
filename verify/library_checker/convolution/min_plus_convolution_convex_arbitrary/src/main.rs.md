---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
    title: crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/convex_and_convex.rs
    title: crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - icon: ':heavy_check_mark:'
    path: crates/convolution/minplus_convolution/src/lib.rs
    title: crates/convolution/minplus_convolution/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary
    links:
    - https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/min_plus_convolution_convex_arbitrary\n\
    \nuse minplus_convolution::minplus_convolution_convex_and_arbitrary;\nuse proconio::fastout;\n\
    use proconio::input;\n\n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n\
    \        m: usize,\n        a: [i64; n],\n        b: [i64; m],\n    }\n    let\
    \ c = minplus_convolution_convex_and_arbitrary(&a, &b);\n    for &x in &c {\n\
    \        print!(\"{} \", x);\n    }\n    println!();\n}\n"
  dependsOn:
  - crates/convolution/minplus_convolution/src/concave_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_arbitrary.rs
  - crates/convolution/minplus_convolution/src/convex_and_convex.rs
  - crates/convolution/minplus_convolution/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
  requiredBy: []
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
- /verify/verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs.html
title: verify/library_checker/convolution/min_plus_convolution_convex_arbitrary/src/main.rs
---
