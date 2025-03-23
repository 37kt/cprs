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
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/constructor.rs
    title: crates/polynomial/formal_power_series/src/constructor.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/convert.rs
    title: crates/polynomial/formal_power_series/src/convert.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/exp.rs
    title: crates/polynomial/formal_power_series/src/exp.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/inv.rs
    title: crates/polynomial/formal_power_series/src/inv.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/lib.rs
    title: crates/polynomial/formal_power_series/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/log.rs
    title: crates/polynomial/formal_power_series/src/log.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/mul.rs
    title: crates/polynomial/formal_power_series/src/mul.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/ops.rs
    title: crates/polynomial/formal_power_series/src/ops.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/pow.rs
    title: crates/polynomial/formal_power_series/src/pow.rs
  - icon: ':heavy_check_mark:'
    path: crates/polynomial/formal_power_series/src/sqrt.rs
    title: crates/polynomial/formal_power_series/src/sqrt.rs
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
  code: "use macros::{chmax, mvec, yes};\nuse proconio::fastout;\n\n#[fastout]\nfn\
    \ main() {\n    let mut x = 10;\n    chmax!(x, 20);\n    println!(\"{}\", x);\n\
    \n    let a = mvec![1; 10; 10];\n\n    println!(\"{:?}\", a);\n\n    yes!(a[0][0]\
    \ == 1);\n}\n"
  dependsOn:
  - crates/misc/macros/src/chminmax.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/mvec.rs
  - crates/misc/macros/src/yes.rs
  - crates/polynomial/formal_power_series/src/constructor.rs
  - crates/polynomial/formal_power_series/src/convert.rs
  - crates/polynomial/formal_power_series/src/exp.rs
  - crates/polynomial/formal_power_series/src/inv.rs
  - crates/polynomial/formal_power_series/src/lib.rs
  - crates/polynomial/formal_power_series/src/log.rs
  - crates/polynomial/formal_power_series/src/mul.rs
  - crates/polynomial/formal_power_series/src/ops.rs
  - crates/polynomial/formal_power_series/src/pow.rs
  - crates/polynomial/formal_power_series/src/sqrt.rs
  isVerificationFile: false
  path: verify/sandbox/test/src/main.rs
  requiredBy: []
  timestamp: '2025-03-21 01:12:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: verify/sandbox/test/src/main.rs
layout: document
redirect_from:
- /library/verify/sandbox/test/src/main.rs
- /library/verify/sandbox/test/src/main.rs.html
title: verify/sandbox/test/src/main.rs
---
