---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/math/two-satisfiability/src/lib.rs
    title: crates/math/two-satisfiability/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/two_sat
    links:
    - https://judge.yosupo.jp/problem/two_sat
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.8/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat\n\
    \nuse proconio::{fastout, input};\nuse two_satisfiability::TwoSatisfiability;\n\
    \n#[fastout]\nfn main() {\n    input! {\n        _: String,\n        _: String,\n\
    \        n: usize,\n        m: usize,\n    }\n    let mut ts = TwoSatisfiability::new(n);\n\
    \    for _ in 0..m {\n        input! {\n            i: i64,\n            j: i64,\n\
    \            _: i64,\n        }\n        let (i, f) = if i < 0 {\n           \
    \ ((-i - 1) as usize, false)\n        } else {\n            ((i - 1) as usize,\
    \ true)\n        };\n        let (j, g) = if j < 0 {\n            ((-j - 1) as\
    \ usize, false)\n        } else {\n            ((j - 1) as usize, true)\n    \
    \    };\n        ts.or(i, f, j, g);\n    }\n    if let Some(res) = ts.solve()\
    \ {\n        println!(\"s SATISFIABLE\");\n        print!(\"v \");\n        for\
    \ i in 0..n {\n            if !res[i] {\n                print!(\"-\");\n    \
    \        }\n            print!(\"{} \", i + 1);\n        }\n        println!(\"\
    0\");\n    } else {\n        println!(\"s UNSATISFIABLE\");\n    }\n}\n"
  dependsOn:
  - crates/math/two-satisfiability/src/lib.rs
  isVerificationFile: true
  path: verify/two_sat/src/main.rs
  requiredBy: []
  timestamp: '2024-12-27 04:46:01+00:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/two_sat/src/main.rs
layout: document
redirect_from:
- /verify/verify/two_sat/src/main.rs
- /verify/verify/two_sat/src/main.rs.html
title: verify/two_sat/src/main.rs
---
