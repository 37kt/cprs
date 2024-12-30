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
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "/// \u30D3\u30C3\u30C8\u3067\u8868\u73FE\u3055\u308C\u305F\u96C6\u5408 s\
    \ \u306E\u90E8\u5206\u96C6\u5408\u3092\u6607\u9806\u306B\u5217\u6319\npub fn bit_subsets(s:\
    \ usize) -> impl Iterator<Item = usize> {\n    let mut x = 0;\n    std::iter::from_fn(move\
    \ || {\n        let res = x;\n        if res == !0 {\n            None\n     \
    \   } else if x == s {\n            x = !0;\n            Some(res)\n        }\
    \ else {\n            x = x.wrapping_sub(s) & s;\n            Some(res)\n    \
    \    }\n    })\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/misc/bitutil/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/misc/bitutil/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/bitutil/src/lib.rs
- /library/crates/misc/bitutil/src/lib.rs.html
title: crates/misc/bitutil/src/lib.rs
---
