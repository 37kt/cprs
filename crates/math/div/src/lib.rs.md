---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/rational/src/lib.rs
    title: crates/algebraic/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/static-range-inversions-query/src/lib.rs
    title: crates/data-structure/static-range-inversions-query/src/lib.rs
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use algebraic::{One, Zero};\nuse std::ops::{Add, BitXor, Div, Rem, Sub};\n\
    \npub fn div_ceil<T>(a: T, b: T) -> T\nwhere\n    T: Copy\n        + Zero\n  \
    \      + One\n        + Add<Output = T>\n        + Div<Output = T>\n        +\
    \ Rem<Output = T>\n        + BitXor<Output = T>\n        + PartialOrd,\n{\n  \
    \  let zero = T::zero();\n    let one = T::one();\n    a / b\n        + if a ^\
    \ b >= zero && a % b != zero {\n            one\n        } else {\n          \
    \  zero\n        }\n}\n\npub fn div_floor<T>(a: T, b: T) -> T\nwhere\n    T: Copy\n\
    \        + Zero\n        + One\n        + Sub<Output = T>\n        + Div<Output\
    \ = T>\n        + Rem<Output = T>\n        + BitXor<Output = T>\n        + PartialOrd,\n\
    {\n    let zero = T::zero();\n    let one = T::one();\n    a / b\n        - if\
    \ a ^ b < zero && a % b != zero {\n            one\n        } else {\n       \
    \     zero\n        }\n}\n\n#[cfg(test)]\nmod tests {\n    use super::*;\n\n \
    \   #[test]\n    fn test_div_floor() {\n        assert_eq!(div_floor(10, 3), 3);\n\
    \        assert_eq!(div_floor(10, -3), -4);\n        assert_eq!(div_floor(-10,\
    \ 3), -4);\n        assert_eq!(div_floor(-10, -3), 3);\n    }\n\n    #[test]\n\
    \    fn test_div_ceil() {\n        assert_eq!(div_ceil(10, 3), 4);\n        assert_eq!(div_ceil(10,\
    \ -3), -3);\n        assert_eq!(div_ceil(-10, 3), -3);\n        assert_eq!(div_ceil(-10,\
    \ -3), 4);\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  isVerificationFile: false
  path: crates/math/div/src/lib.rs
  requiredBy:
  - crates/algebraic/rational/src/lib.rs
  - crates/data-structure/static-range-inversions-query/src/lib.rs
  timestamp: '2024-12-22 00:14:04+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/div/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/div/src/lib.rs
- /library/crates/math/div/src/lib.rs.html
title: crates/math/div/src/lib.rs
---
