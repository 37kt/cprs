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
    \ isize i8 i16 i32 i64 i128);\n\n/// [l..r] \u3067\u6700\u5C0F\u5024\u3092\u3068\
    \u308B x \u3068 f(x) \u3092\u8FD4\u3059\npub fn ternary_search<I, T>(mut l: I,\
    \ mut r: I, mut f: impl FnMut(I) -> T) -> (I, T)\nwhere\n    I: Copy + Add<Output\
    \ = I> + Sub<Output = I> + Div<Output = I> + Zero + One + PartialOrd,\n    T:\
    \ Copy + PartialOrd,\n{\n    assert!(l <= r);\n    let one = I::one();\n    let\
    \ two = one + one;\n    let three = two + one;\n    while l + two < r {\n    \
    \    let m1 = (l + l + r) / three;\n        let m2 = (l + r + r) / three;\n  \
    \      if f(m1) < f(m2) {\n            r = m2;\n        } else {\n           \
    \ l = m1;\n        }\n    }\n    let mut mn = f(l);\n    let mut i = l;\n    if\
    \ l + one <= r && f(l + one) < mn {\n        mn = f(l + one);\n        i = l +\
    \ one;\n    }\n    if l + two <= r && f(l + two) < mn {\n        mn = f(l + two);\n\
    \        i = l + two;\n    }\n    (i, mn)\n}\n\n/// [l..r] \u3067\u6700\u5C0F\u5024\
    \u3092\u3068\u308B x \u3068 f(x) \u3092\u8FD4\u3059\npub fn ternary_search_f64<T>(mut\
    \ l: f64, mut r: f64, mut f: impl FnMut(f64) -> T) -> (f64, T)\nwhere\n    T:\
    \ Copy + PartialOrd,\n{\n    for _ in 0..100 {\n        let m1 = (l * 2.0 + r)\
    \ / 3.0;\n        let m2 = (l + 2.0 * r) / 3.0;\n        if f(m1) < f(m2) {\n\
    \            r = m2;\n        } else {\n            l = m1;\n        }\n    }\n\
    \    (l, f(l))\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/algorithm/ternary-search/src/lib.rs
  requiredBy: []
  timestamp: '2024-08-15 16:42:08+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/algorithm/ternary-search/src/lib.rs
layout: document
redirect_from:
- /library/crates/algorithm/ternary-search/src/lib.rs
- /library/crates/algorithm/ternary-search/src/lib.rs.html
title: crates/algorithm/ternary-search/src/lib.rs
---
