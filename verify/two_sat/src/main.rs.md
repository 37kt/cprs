---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/two_sat
    links:
    - https://judge.yosupo.jp/problem/two_sat
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.6/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat\n\
    \nuse proconio::input;\nuse two_satisfiability::TwoSatisfiability;\n\n#[proconio::fastout]\n\
    fn main() {\n    input! {\n        _: String,\n        _: String,\n        n:\
    \ usize,\n        m: usize,\n    }\n    let mut ts = TwoSatisfiability::new(n);\n\
    \    for _ in 0..m {\n        input! {\n            a: i64,\n            b: i64,\n\
    \            _: i64,\n        }\n        let a = if a < 0 { a } else { a - 1 }\
    \ as usize;\n        let b = if b < 0 { b } else { b - 1 } as usize;\n       \
    \ ts.add(a, b);\n    }\n    if let Some(res) = ts.solve() {\n        println!(\"\
    s SATISFIABLE\");\n        print!(\"v \");\n        for i in 0..n {\n        \
    \    if !res[i] {\n                print!(\"-\");\n            }\n           \
    \ print!(\"{} \", i + 1);\n        }\n        println!(\"0\");\n    } else {\n\
    \        println!(\"s UNSATISFIABLE\");\n    }\n}\n"
  dependsOn: []
  isVerificationFile: true
  path: verify/two_sat/src/main.rs
  requiredBy: []
  timestamp: '1970-01-01 00:00:00+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/two_sat/src/main.rs
layout: document
redirect_from:
- /verify/verify/two_sat/src/main.rs
- /verify/verify/two_sat/src/main.rs.html
title: verify/two_sat/src/main.rs
---
