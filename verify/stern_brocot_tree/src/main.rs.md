---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/rational/src/lib.rs
    title: crates/algebraic/rational/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/math/stern-brocot-tree/src/lib.rs
    title: crates/math/stern-brocot-tree/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.yosupo.jp/problem/stern_brocot_tree
    links:
    - https://judge.yosupo.jp/problem/stern_brocot_tree
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/stern_brocot_tree\n\
    \nuse proconio::input;\nuse rational::Rational;\nuse stern_brocot_tree::SternBrocotTreeNode;\n\
    \nfn main() {\n    input! {\n        t: usize,\n    }\n    for _ in 0..t {\n \
    \       input! {\n            ty: String,\n        }\n        match ty.as_str()\
    \ {\n            \"ENCODE_PATH\" => {\n                input! {\n            \
    \        a: i128,\n                    b: i128,\n                }\n         \
    \       let sbt = SternBrocotTreeNode::new(Rational::new(a, b));\n           \
    \     let path = sbt.path();\n                print!(\"{}\", path.len());\n  \
    \              for x in path {\n                    if x < 0 {\n             \
    \           print!(\" L {}\", -x);\n                    } else {\n           \
    \             print!(\" R {}\", x);\n                    }\n                }\n\
    \                println!();\n            }\n            \"DECODE_PATH\" => {\n\
    \                input! {\n                    k: usize,\n                }\n\
    \                let mut sbt = SternBrocotTreeNode::default();\n             \
    \   for _ in 0..k {\n                    input! {\n                        c:\
    \ String,\n                        n: i128,\n                    }\n         \
    \           sbt.go_child(if c == \"L\" { -n } else { n });\n                }\n\
    \                let Rational { num, den } = sbt.val();\n                println!(\"\
    {} {}\", num, den);\n            }\n            \"LCA\" => {\n               \
    \ input! {\n                    a: i128,\n                    b: i128,\n     \
    \               c: i128,\n                    d: i128,\n                }\n  \
    \              let sbt1 = SternBrocotTreeNode::new(Rational::new(a, b));\n   \
    \             let sbt2 = SternBrocotTreeNode::new(Rational::new(c, d));\n    \
    \            let lca = sbt1.lca(&sbt2);\n                let Rational { num, den\
    \ } = lca.val();\n                println!(\"{} {}\", num, den);\n           \
    \ }\n            \"ANCESTOR\" => {\n                input! {\n               \
    \     k: i128,\n                    a: i128,\n                    b: i128,\n \
    \               }\n                let mut sbt = SternBrocotTreeNode::new(Rational::new(a,\
    \ b));\n                let d = sbt.depth();\n                if d < k {\n   \
    \                 println!(\"-1\");\n                } else {\n              \
    \      sbt.go_parent(d - k);\n                    let Rational { num, den } =\
    \ sbt.val();\n                    println!(\"{} {}\", num, den);\n           \
    \     }\n            }\n            \"RANGE\" => {\n                input! {\n\
    \                    a: i128,\n                    b: i128,\n                }\n\
    \                let sbt = SternBrocotTreeNode::new(Rational::new(a, b));\n  \
    \              let Rational { num: f, den: g } = sbt.lower_bound();\n        \
    \        let Rational { num: h, den: k } = sbt.upper_bound();\n              \
    \  println!(\"{} {} {} {}\", f, g, h, k);\n            }\n            _ => unreachable!(),\n\
    \        }\n    }\n}\n"
  dependsOn:
  - crates/algebraic/rational/src/lib.rs
  - crates/math/stern-brocot-tree/src/lib.rs
  isVerificationFile: true
  path: verify/stern_brocot_tree/src/main.rs
  requiredBy: []
  timestamp: '2024-03-11 13:55:24+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: verify/stern_brocot_tree/src/main.rs
layout: document
redirect_from:
- /verify/verify/stern_brocot_tree/src/main.rs
- /verify/verify/stern_brocot_tree/src/main.rs.html
title: verify/stern_brocot_tree/src/main.rs
---
