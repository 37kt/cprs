---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/w-ary-tree-set/src/lib.rs
    title: crates/data-structure/w-ary-tree-set/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/predecessor_problem
    links:
    - https://judge.yosupo.jp/problem/predecessor_problem
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem\n\
    \nuse proconio::{input, marker::Bytes};\nuse w_ary_tree_set::WAryTreeSet;\n\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n        t: Bytes,\n\
    \    }\n    let mut tr = WAryTreeSet::new(10_000_000);\n    for i in 0..n {\n\
    \        if t[i] == b'1' {\n            tr.insert(i);\n        }\n    }\n    for\
    \ _ in 0..q {\n        input! {\n            ty: usize,\n            k: usize,\n\
    \        }\n        match ty {\n            0 => {\n                tr.insert(k);\n\
    \            }\n            1 => {\n                tr.remove(k);\n          \
    \  }\n            2 => {\n                println!(\"{}\", tr.contains(k) as usize);\n\
    \            }\n            3 => {\n                if let Some(x) = tr.next(k)\
    \ {\n                    println!(\"{}\", x);\n                } else {\n    \
    \                println!(\"-1\");\n                }\n            }\n       \
    \     _ => {\n                if let Some(x) = tr.prev(k) {\n                \
    \    println!(\"{}\", x);\n                } else {\n                    println!(\"\
    -1\");\n                }\n            }\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/w-ary-tree-set/src/lib.rs
  isVerificationFile: true
  path: verify/predecessor_problem/src/main.rs
  requiredBy: []
  timestamp: '2023-05-13 18:34:42+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/predecessor_problem/src/main.rs
layout: document
redirect_from:
- /verify/verify/predecessor_problem/src/main.rs
- /verify/verify/predecessor_problem/src/main.rs.html
title: verify/predecessor_problem/src/main.rs
---
