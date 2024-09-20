---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/counting_spanning_tree_directed
    links:
    - https://judge.yosupo.jp/problem/counting_spanning_tree_directed
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/counting_spanning_tree_directed\n\
    \nuse count_spanning_tree_directed::count_spanning_tree_directed;\nuse modint::ModInt998244353\
    \ as Mint;\nuse proconio::input;\n\n#[proconio::fastout]\nfn main() {\n    input!\
    \ {\n        n: usize,\n        m: usize,\n        r: usize,\n        uv: [(usize,\
    \ usize); m],\n    }\n    let mut g = vec![vec![0; n]; n];\n    for &(u, v) in\
    \ &uv {\n        g[u][v] += 1;\n    }\n    let res: Mint = count_spanning_tree_directed(&g,\
    \ r);\n    println!(\"{}\", res);\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/count_spanning_tree_directed/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/count_spanning_tree_directed/src/main.rs
layout: document
redirect_from:
- /verify/verify/count_spanning_tree_directed/src/main.rs
- /verify/verify/count_spanning_tree_directed/src/main.rs.html
title: verify/count_spanning_tree_directed/src/main.rs
---
