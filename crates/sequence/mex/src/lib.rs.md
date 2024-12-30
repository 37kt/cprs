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
  code: "/// a \u306B\u542B\u307E\u308C\u306A\u3044\u6700\u5C0F\u306E\u975E\u8CA0\u6574\
    \u6570\u3092\u6C42\u3081\u308B\u3002\npub fn mex(a: &[usize]) -> usize {\n   \
    \ let mut a = a.to_vec();\n    for i in 0..a.len() {\n        while i < a.len()\
    \ && a[i] != i {\n            if a[i] >= a.len() || a[i] == a[a[i]] {\n      \
    \          a.swap_remove(i);\n            } else {\n                let j = a[i];\n\
    \                a.swap(i, j);\n            }\n        }\n    }\n    a.iter()\n\
    \        .enumerate()\n        .position(|(i, &x)| x != i)\n        .unwrap_or(a.len())\n\
    }\n\n#[cfg(test)]\nmod tests {\n    use std::collections::BTreeSet;\n\n    use\
    \ rand::Rng;\n\n    use super::*;\n\n    #[test]\n    fn test_mex() {\n      \
    \  for _ in 0..1000 {\n            let mut rng = rand::thread_rng();\n       \
    \     let n = rng.gen_range(0..100);\n            let m = n + 10;\n          \
    \  let a = (0..n).map(|_| rng.gen_range(0..m)).collect::<Vec<_>>();\n        \
    \    let res = mex(&a);\n\n            let mut st = BTreeSet::new();\n       \
    \     for i in 0..=m {\n                st.insert(i);\n            }\n       \
    \     for &x in &a {\n                st.remove(&x);\n            }\n        \
    \    assert_eq!(res, *st.iter().next().unwrap());\n        }\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/sequence/mex/src/lib.rs
  requiredBy: []
  timestamp: '2024-12-30 09:13:10+00:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: crates/sequence/mex/src/lib.rs
layout: document
redirect_from:
- /library/crates/sequence/mex/src/lib.rs
- /library/crates/sequence/mex/src/lib.rs.html
title: crates/sequence/mex/src/lib.rs
---
