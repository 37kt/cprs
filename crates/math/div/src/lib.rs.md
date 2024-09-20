---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, BitXor, Div, Rem, Sub};\n\npub trait Zero {\n    fn zero()\
    \ -> Self;\n    fn is_zero(&self) -> bool;\n}\n\npub trait One {\n    fn one()\
    \ -> Self;\n    fn is_one(&self) -> bool;\n}\n\nmacro_rules! impl_zero_one {\n\
    \    ($($t:ty)*) => {\n        $(\n            impl $crate::Zero for $t {\n  \
    \              fn zero() -> Self {\n                    0\n                }\n\
    \                fn is_zero(&self) -> bool {\n                    *self == 0\n\
    \                }\n            }\n            impl $crate::One for $t {\n   \
    \             fn one() -> Self {\n                    1\n                }\n \
    \               fn is_one(&self) -> bool {\n                    *self == 1\n \
    \               }\n            }\n        )*\n    };\n}\n\nimpl_zero_one!(usize\
    \ u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);\n\npub fn div_ceil<T>(a: T,\
    \ b: T) -> T\nwhere\n    T: Copy\n        + Zero\n        + One\n        + Add<Output\
    \ = T>\n        + Div<Output = T>\n        + Rem<Output = T>\n        + BitXor<Output\
    \ = T>\n        + PartialOrd,\n{\n    let zero = T::zero();\n    let one = T::one();\n\
    \    a / b\n        + if a ^ b >= zero && a % b != zero {\n            one\n \
    \       } else {\n            zero\n        }\n}\n\npub fn div_floor<T>(a: T,\
    \ b: T) -> T\nwhere\n    T: Copy\n        + Zero\n        + One\n        + Sub<Output\
    \ = T>\n        + Div<Output = T>\n        + Rem<Output = T>\n        + BitXor<Output\
    \ = T>\n        + PartialOrd,\n{\n    let zero = T::zero();\n    let one = T::one();\n\
    \    a / b\n        - if a ^ b < zero && a % b != zero {\n            one\n  \
    \      } else {\n            zero\n        }\n}\n\n#[cfg(test)]\nmod tests {\n\
    \    use super::*;\n\n    #[test]\n    fn test_div_floor() {\n        assert_eq!(div_floor(10,\
    \ 3), 3);\n        assert_eq!(div_floor(10, -3), -4);\n        assert_eq!(div_floor(-10,\
    \ 3), -4);\n        assert_eq!(div_floor(-10, -3), 3);\n    }\n\n    #[test]\n\
    \    fn test_div_ceil() {\n        assert_eq!(div_ceil(10, 3), 4);\n        assert_eq!(div_ceil(10,\
    \ -3), -3);\n        assert_eq!(div_ceil(-10, 3), -3);\n        assert_eq!(div_ceil(-10,\
    \ -3), 4);\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/div/src/lib.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/math/div/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/div/src/lib.rs
- /library/crates/math/div/src/lib.rs.html
title: crates/math/div/src/lib.rs
---
