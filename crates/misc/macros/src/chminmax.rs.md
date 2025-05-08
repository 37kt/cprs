---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/yes.rs
    title: crates/misc/macros/src/yes.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/lib.rs
    title: crates/misc/macros/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/misc/macros/src/mvec.rs
    title: crates/misc/macros/src/mvec.rs
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
  code: "#[macro_export]\nmacro_rules! min {\n    ($a:expr $(,)*) => {{\n        $a\n\
    \    }};\n    ($a:expr, $b:expr $(,)*) => {{\n        std::cmp::min($a, $b)\n\
    \    }};\n    ($a:expr, $($rest:expr),+ $(,)*) => {{\n        std::cmp::min($a,\
    \ $crate::min!($($rest),+))\n    }};\n}\n\n#[macro_export]\nmacro_rules! max {\n\
    \    ($a:expr $(,)*) => {{\n        $a\n    }};\n    ($a:expr, $b:expr $(,)*)\
    \ => {{\n        std::cmp::max($a, $b)\n    }};\n    ($a:expr, $($rest:expr),+\
    \ $(,)*) => {{\n        std::cmp::max($a, $crate::max!($($rest),+))\n    }};\n\
    }\n\n#[macro_export]\nmacro_rules! chmin {\n    ($base:expr, $($cmps:expr),+ $(,)*)\
    \ => {{\n        let cmp_min = $crate::min!($($cmps),+);\n        if $base > cmp_min\
    \ {\n            $base = cmp_min;\n            true\n        } else {\n      \
    \      false\n        }\n    }};\n}\n\n#[macro_export]\nmacro_rules! chmax {\n\
    \    ($base:expr, $($cmps:expr),+ $(,)*) => {{\n        let cmp_max = $crate::max!($($cmps),+);\n\
    \        if $base < cmp_max {\n            $base = cmp_max;\n            true\n\
    \        } else {\n            false\n        }\n    }};\n}\n"
  dependsOn:
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/mvec.rs
  - crates/misc/macros/src/yes.rs
  isVerificationFile: false
  path: crates/misc/macros/src/chminmax.rs
  requiredBy:
  - verify/sandbox/test/src/main.rs
  - crates/misc/macros/src/lib.rs
  - crates/misc/macros/src/yes.rs
  - crates/misc/macros/src/mvec.rs
  timestamp: '2025-03-21 00:58:56+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/yukicoder/dp/yuki0733_bitdp/src/main.rs
documentation_of: crates/misc/macros/src/chminmax.rs
layout: document
redirect_from:
- /library/crates/misc/macros/src/chminmax.rs
- /library/crates/misc/macros/src/chminmax.rs.html
title: crates/misc/macros/src/chminmax.rs
---
