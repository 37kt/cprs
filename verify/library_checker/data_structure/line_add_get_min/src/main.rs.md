---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
    title: crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/line_add_get_min
    links:
    - https://judge.yosupo.jp/problem/line_add_get_min
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/line_add_get_min\n\
    \nuse proconio::fastout;\nuse proconio::input;\n\nuse offline_dynamic_convex_hull_trick::OfflineDynamicConvexHullTrick;\n\
    use proconio::read_value;\n\n#[fastout]\nfn main() {\n    input! {\n        n:\
    \ usize,\n        q: usize,\n    }\n    let mut cht = OfflineDynamicConvexHullTrick::new();\n\
    \    for i in 0..n + q {\n        if i < n || read_value!(usize) == 0 {\n    \
    \        input! {\n                a: i64,\n                b: i64,\n        \
    \    }\n            cht.add_line(a, b);\n        } else {\n            input!\
    \ {\n                x: i64,\n            }\n            cht.min(x);\n       \
    \ }\n    }\n    for y in cht.solve() {\n        println!(\"{}\", y);\n    }\n\
    }\n"
  dependsOn:
  - crates/data_structure/offline_dynamic_convex_hull_trick/src/lib.rs
  isVerificationFile: true
  path: verify/library_checker/data_structure/line_add_get_min/src/main.rs
  requiredBy: []
  timestamp: '2025-04-06 02:35:23+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/library_checker/data_structure/line_add_get_min/src/main.rs
layout: document
redirect_from:
- /verify/verify/library_checker/data_structure/line_add_get_min/src/main.rs
- /verify/verify/library_checker/data_structure/line_add_get_min/src/main.rs.html
title: verify/library_checker/data_structure/line_add_get_min/src/main.rs
---
