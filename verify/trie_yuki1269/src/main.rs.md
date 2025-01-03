---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/number-theory/modint/src/lib.rs
    title: crates/number-theory/modint/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/string/trie/src/lib.rs
    title: crates/string/trie/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://yukicoder.me/problems/no/1269
    links:
    - https://yukicoder.me/problems/no/1269
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://yukicoder.me/problems/no/1269\n\n\
    use modint::ModInt1000000007 as Mint;\n#[allow(unused_imports)]\nuse proconio::{\n\
    \    input,\n    marker::{Bytes, Chars, Usize1},\n};\nuse trie::Trie;\n\nfn main()\
    \ {\n    input! {\n        n: usize,\n        l: usize,\n        r: usize,\n \
    \   }\n\n    let to_array = |mut x: usize| {\n        let mut res = vec![];\n\
    \        while x > 0 {\n            res.push(x % 10);\n            x /= 10;\n\
    \        }\n        res.reverse();\n        res\n    };\n\n    let mut trie =\
    \ Trie::<10>::new();\n    let mut a0 = 1;\n    let mut a1 = 1;\n    while a1 <=\
    \ r {\n        if l <= a1 {\n            trie.add(&to_array(a1));\n        }\n\
    \        (a0, a1) = (a1, a0 + a1);\n    }\n    trie.build();\n\n    let mut dp\
    \ = vec![Mint::new(0); trie.len()];\n    dp[0] += 1;\n    for _ in 0..n {\n  \
    \      let mut ndp = vec![Mint::new(0); trie.len()];\n        for v in 0..trie.len()\
    \ {\n            for c in 0..10 {\n                let u = trie.next(v, c);\n\
    \                if trie.count(u) == 0 {\n                    ndp[u] += dp[v];\n\
    \                }\n            }\n        }\n        dp = ndp;\n    }\n\n   \
    \ let res = dp.iter().sum::<Mint>() - 1;\n    println!(\"{}\", res);\n}\n"
  dependsOn:
  - crates/number-theory/modint/src/lib.rs
  - crates/string/trie/src/lib.rs
  isVerificationFile: true
  path: verify/trie_yuki1269/src/main.rs
  requiredBy: []
  timestamp: '2025-01-03 08:24:26+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/trie_yuki1269/src/main.rs
layout: document
redirect_from:
- /verify/verify/trie_yuki1269/src/main.rs
- /verify/verify/trie_yuki1269/src/main.rs.html
title: verify/trie_yuki1269/src/main.rs
---
