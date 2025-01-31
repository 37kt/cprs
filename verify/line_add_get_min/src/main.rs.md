---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/li-chao-tree-dynamic/src/lib.rs
    title: crates/data-structure/li-chao-tree-dynamic/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/line_add_get_min
    links:
    - https://judge.yosupo.jp/problem/line_add_get_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.1/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min\n\
    \nuse li_chao_tree_dynamic::LiChaoTreeDynamic;\nuse proconio::input;\n\nconst\
    \ MAX: i64 = 1_000_000_000;\n\n#[proconio::fastout]\nfn main() {\n    input! {\n\
    \        n: usize,\n        q: usize,\n    }\n    let mut lct = LiChaoTreeDynamic::new(-MAX,\
    \ MAX, false);\n    for _ in 0..n {\n        input! {\n            a: i64,\n \
    \           b: i64,\n        }\n        lct.add_line(a, b);\n    }\n    for _\
    \ in 0..q {\n        input! {\n            ty: usize,\n        }\n        if ty\
    \ == 0 {\n            input! {\n                a: i64,\n                b: i64,\n\
    \            }\n            lct.add_line(a, b);\n        } else {\n          \
    \  input! {\n                p: i64,\n            }\n            println!(\"{}\"\
    , lct.find(p).unwrap());\n        }\n    }\n}\n"
  dependsOn:
  - crates/data-structure/li-chao-tree-dynamic/src/lib.rs
  isVerificationFile: true
  path: verify/line_add_get_min/src/main.rs
  requiredBy: []
  timestamp: '2023-05-16 16:25:17+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/line_add_get_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/line_add_get_min/src/main.rs
- /verify/verify/line_add_get_min/src/main.rs.html
title: verify/line_add_get_min/src/main.rs
---
