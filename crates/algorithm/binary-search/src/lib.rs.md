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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::{Add, Div, Sub};\n\npub trait Zero {\n    fn zero() -> Self;\n\
    \    fn is_zero(&self) -> bool;\n}\n\npub trait One {\n    fn one() -> Self;\n\
    \    fn is_one(&self) -> bool;\n}\n\nmacro_rules! impl_zero_one {\n    ($($t:ty)*)\
    \ => {\n        $(\n            impl $crate::Zero for $t {\n                fn\
    \ zero() -> Self {\n                    0\n                }\n               \
    \ fn is_zero(&self) -> bool {\n                    *self == 0\n              \
    \  }\n            }\n            impl $crate::One for $t {\n                fn\
    \ one() -> Self {\n                    1\n                }\n                fn\
    \ is_one(&self) -> bool {\n                    *self == 1\n                }\n\
    \            }\n        )*\n    };\n}\n\nimpl_zero_one!(usize u8 u16 u32 u64 u128\
    \ isize i8 i16 i32 i64 i128);\n\n/// [l..r) \u3067 f(x) = false \u3068\u306A\u308B\
    \u6700\u5C0F\u306E x \u3092\u8FD4\u3059\n/// \u306A\u3044\u5834\u5408\u306F r\
    \ \u3092\u8FD4\u3059\npub fn binary_search<I>(mut l: I, mut r: I, mut f: impl\
    \ FnMut(I) -> bool) -> I\nwhere\n    I: Copy + Add<Output = I> + Sub<Output =\
    \ I> + Div<Output = I> + Zero + One + PartialOrd,\n{\n    let one = I::one();\n\
    \    let two = one + one;\n    if !f(l) {\n        return l;\n    }\n    while\
    \ l + one < r {\n        let m = (l + r) / two;\n        if f(m) {\n         \
    \   l = m;\n        } else {\n            r = m;\n        }\n    }\n    r\n}\n\
    \n/// [l..r] \u3067 f(x) = false \u3068\u306A\u308B\u6700\u5C0F\u306E x \u3092\
    \u8FD4\u3059\npub fn binary_search_f64(mut l: f64, mut r: f64, mut f: impl FnMut(f64)\
    \ -> bool) -> f64 {\n    for _ in 0..100 {\n        let m = (l + r) / 2.0;\n \
    \       if f(m) {\n            l = m;\n        } else {\n            r = m;\n\
    \        }\n    }\n    r\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/binary-search/src/lib.rs
  requiredBy: []
  timestamp: '2024-08-15 16:42:08+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/binary-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/binary-search/src/lib.rs
- /library/crates/algorithm/binary-search/src/lib.rs.html
title: crates/algorithm/binary-search/src/lib.rs
---
