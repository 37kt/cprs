---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/persistent-union-find/src/lib.rs
    title: crates/data-structure/persistent-union-find/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/persistent_unionfind
    links:
    - https://judge.yosupo.jp/problem/persistent_unionfind
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.7/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/persistent_unionfind\n\
    \nuse persistent_union_find::PersistentUnionFind;\nuse proconio::input;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        n: usize,\n        q: usize,\n    }\n    let\
    \ mut uf = vec![PersistentUnionFind::new(n); q + 1];\n    for i in 1..=q {\n \
    \       input! {\n            t: usize,\n            k: isize,\n            u:\
    \ usize,\n            v: usize,\n        }\n        let k = (k + 1) as usize;\n\
    \        if t == 0 {\n            uf[i] = uf[k].merge(u, v);\n        } else {\n\
    \            println!(\"{}\", uf[k].same(u, v) as usize);\n        }\n    }\n\
    }\n"
  dependsOn:
  - crates/data-structure/persistent-union-find/src/lib.rs
  isVerificationFile: true
  path: verify/persistent_unionfind/src/main.rs
  requiredBy: []
  timestamp: '2023-05-08 17:12:04+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/persistent_unionfind/src/main.rs
layout: document
redirect_from:
- /verify/verify/persistent_unionfind/src/main.rs
- /verify/verify/persistent_unionfind/src/main.rs.html
title: verify/persistent_unionfind/src/main.rs
---
