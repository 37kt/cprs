---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/cartesian_tree/src/main.rs
    title: verify/cartesian_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "pub fn cartesian_tree<T>(a: &[T]) -> Vec<usize>\nwhere\n    T: Ord,\n{\n\
    \    let n = a.len();\n    let mut par = vec![!0; n];\n    let mut st = vec![];\n\
    \    for i in 0..n {\n        let mut l = !0;\n        while !st.is_empty() &&\
    \ a[*st.last().unwrap()] >= a[i] {\n            l = st.pop().unwrap();\n     \
    \   }\n        if !st.is_empty() {\n            par[i] = st[st.len() - 1];\n \
    \       }\n        if l != !0 {\n            par[l] = i;\n        }\n        st.push(i);\n\
    \    }\n    par\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/tree/cartesian-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-14 16:40:29+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/cartesian_tree/src/main.rs
documentation_of: crates/tree/cartesian-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/tree/cartesian-tree/src/lib.rs
- /library/crates/tree/cartesian-tree/src/lib.rs.html
title: crates/tree/cartesian-tree/src/lib.rs
---
