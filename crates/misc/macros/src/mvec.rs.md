---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/chminmax.rs
    title: crates/misc/macros/src/chminmax.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/chminmax.rs
    title: crates/misc/macros/src/chminmax.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  - icon: ':warning:'
    path: verify/sandbox/test/src/main.rs
    title: verify/sandbox/test/src/main.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
    title: verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.3/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// mvec![]\n#[macro_export]\nmacro_rules! mvec {\n    ($x:expr; $n:expr)\
    \ => {\n        vec![$x; $n]\n    };\n    ($x:expr; $n:expr $(; $m:expr)+) =>\
    \ {\n        vec![mvec![$x $(; $m)*]; $n]\n    };\n}\n"
  dependsOn:
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/yes.rs
  isVerificationFile: false
  path: crates/misc/macros/src/mvec.rs
  requiredBy:
  - verify/sandbox/test/src/main.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/yes.rs
  timestamp: '2025-03-21 00:58:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
documentation_of: crates/misc/macros/src/mvec.rs
layout: document
redirect_from:
- /library/crates/misc/macros/src/mvec.rs
- /library/crates/misc/macros/src/mvec.rs.html
title: crates/misc/macros/src/mvec.rs
---
