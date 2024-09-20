---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/cartesian_tree
    links:
    - https://judge.yosupo.jp/problem/cartesian_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree\n\
    \nuse cartesian_tree::cartesian_tree;\nuse itertools::Itertools;\nuse proconio::input;\n\
    \nfn main() {\n    input! {\n        n: usize,\n        a: [usize; n],\n    }\n\
    \    let p = cartesian_tree(&a);\n    println!(\n        \"{}\",\n        p.into_iter()\n\
    \            .enumerate()\n            .map(|(i, x)| if x == !0 { i } else { x\
    \ })\n            .join(\" \")\n    );\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/cartesian_tree/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/cartesian_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/cartesian_tree/src/main.rs
- /verify/verify/cartesian_tree/src/main.rs.html
title: verify/cartesian_tree/src/main.rs
---
