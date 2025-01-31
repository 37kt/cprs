---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/binary-trie/src/lib.rs
    title: crates/data-structure/binary-trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/ordered_set
    links:
    - https://judge.yosupo.jp/problem/ordered_set
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/ordered_set\n\
    \nuse binary_trie::BinaryTrie;\nuse proconio::fastout;\nuse proconio::input;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        n: usize,\n        q: usize,\n\
    \        a: [usize; n],\n    }\n    let mut st = BinaryTrie::<30>::new();\n  \
    \  for &x in &a {\n        st.insert(x, 1);\n    }\n    for _ in 0..q {\n    \
    \    input! {\n            t: usize,\n            x: usize,\n        }\n     \
    \   if t == 0 {\n            if st.count(x) == 0 {\n                st.insert(x,\
    \ 1);\n            }\n        } else if t == 1 {\n            if st.count(x) ==\
    \ 1 {\n                st.remove(x, 1);\n            }\n        } else if t ==\
    \ 2 {\n            let res = st.kth_smallest(.., x - 1).unwrap_or(!0);\n     \
    \       println!(\"{}\", res as i64);\n        } else if t == 3 {\n          \
    \  let res = st.upper_bound(x);\n            println!(\"{}\", res);\n        }\
    \ else if t == 4 {\n            let res = st.kth_largest(..=x, 0).unwrap_or(!0);\n\
    \            println!(\"{}\", res as i64);\n        } else if t == 5 {\n     \
    \       let res = st.kth_smallest(x.., 0).unwrap_or(!0);\n            println!(\"\
    {}\", res as i64);\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/binary-trie/src/lib.rs
  isVerificationFile: true
  path: verify/ordered_set_binary_trie/src/main.rs
  requiredBy: []
  timestamp: '2025-01-04 07:13:06+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/ordered_set_binary_trie/src/main.rs
layout: document
redirect_from:
- /verify/verify/ordered_set_binary_trie/src/main.rs
- /verify/verify/ordered_set_binary_trie/src/main.rs.html
title: verify/ordered_set_binary_trie/src/main.rs
---
