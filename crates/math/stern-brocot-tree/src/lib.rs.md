---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/rational/src/lib.rs
    title: crates/algebraic/rational/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/stern_brocot_tree/src/main.rs
    title: verify/stern_brocot_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.2/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use rational::Rational;\n\npub struct SternBrocotTreeNode {\n    l: Rational,\n\
    \    m: Rational,\n    r: Rational,\n    seq: Vec<i128>,\n}\n\nimpl Default for\
    \ SternBrocotTreeNode {\n    fn default() -> Self {\n        Self {\n        \
    \    l: Rational::new(0, 1),\n            m: Rational::new(1, 1),\n          \
    \  r: Rational::new(1, 0),\n            seq: vec![],\n        }\n    }\n}\n\n\
    impl SternBrocotTreeNode {\n    pub fn new(mut x: Rational) -> Self {\n      \
    \  assert!(x.num > 0 && x.den > 0);\n        x.reduce();\n        let mut n =\
    \ Self::default();\n        while x.num.min(x.den) > 0 {\n            if x.num\
    \ > x.den {\n                let d = x.num / x.den;\n                x.num -=\
    \ x.den * d;\n                n.go_child(d - if x.num == 0 { 1 } else { 0 });\n\
    \            } else {\n                let d = x.den / x.num;\n              \
    \  x.den -= x.num * d;\n                n.go_child(if x.den == 0 { 1 } else {\
    \ 0 } - d);\n            }\n        }\n        n\n    }\n\n    pub fn depth(&self)\
    \ -> i128 {\n        self.seq.iter().map(|&x| x.abs()).sum::<i128>()\n    }\n\n\
    \    pub fn go_child(&mut self, d: i128) {\n        if d == 0 {\n            return;\n\
    \        }\n        if self.seq.is_empty() || (*self.seq.last().unwrap() < 0)\
    \ != (d < 0) {\n            self.seq.push(d);\n        } else {\n            *self.seq.last_mut().unwrap()\
    \ += d;\n        }\n        if d < 0 {\n            self.r.num += self.l.num *\
    \ -d;\n            self.r.den += self.l.den * -d;\n            self.m = Rational\
    \ {\n                num: self.l.num + self.r.num,\n                den: self.l.den\
    \ + self.r.den,\n            };\n        } else {\n            self.l.num += self.r.num\
    \ * d;\n            self.l.den += self.r.den * d;\n            self.m = Rational\
    \ {\n                num: self.l.num + self.r.num,\n                den: self.l.den\
    \ + self.r.den,\n            };\n        }\n    }\n\n    pub fn go_parent(&mut\
    \ self, mut d: i128) -> bool {\n        assert!(d >= 0);\n        if d == 0 {\n\
    \            return true;\n        }\n        while d != 0 {\n            if self.seq.is_empty()\
    \ {\n                return false;\n            }\n            let mut x = self.seq.pop().unwrap();\n\
    \            let d2 = d.min(x.abs());\n            if x < 0 {\n              \
    \  self.m.num -= self.l.num * d2;\n                self.m.den -= self.l.den *\
    \ d2;\n                self.r = Rational {\n                    num: self.m.num\
    \ - self.l.num,\n                    den: self.m.den - self.l.den,\n         \
    \       };\n                x += d2;\n            } else {\n                self.m.num\
    \ -= self.r.num * d2;\n                self.m.den -= self.r.den * d2;\n      \
    \          self.l = Rational {\n                    num: self.m.num - self.r.num,\n\
    \                    den: self.m.den - self.r.den,\n                };\n     \
    \           x -= d2;\n            }\n            d -= d2;\n            if x !=\
    \ 0 {\n                self.seq.push(x);\n            }\n            if d2 ==\
    \ 0 {\n                break;\n            }\n        }\n        true\n    }\n\
    \n    pub fn lca(&self, other: &SternBrocotTreeNode) -> SternBrocotTreeNode {\n\
    \        let mut res = SternBrocotTreeNode::default();\n        for (&x, &y) in\
    \ self.seq.iter().zip(&other.seq) {\n            if (x < 0) != (y < 0) {\n   \
    \             break;\n            }\n            if x < 0 {\n                res.go_child(x.max(y));\n\
    \            } else if x > 0 {\n                res.go_child(x.min(y));\n    \
    \        }\n            if x != y {\n                break;\n            }\n \
    \       }\n        res\n    }\n\n    pub fn lower_bound(&self) -> Rational {\n\
    \        self.l\n    }\n\n    pub fn upper_bound(&self) -> Rational {\n      \
    \  self.r\n    }\n\n    pub fn val(&self) -> Rational {\n        self.m\n    }\n\
    \n    pub fn path(&self) -> Vec<i128> {\n        self.seq.clone()\n    }\n}\n\n\
    impl PartialEq for SternBrocotTreeNode {\n    fn eq(&self, other: &Self) -> bool\
    \ {\n        self.m == other.m\n    }\n}\n\nimpl Eq for SternBrocotTreeNode {}\n\
    \nimpl PartialOrd for SternBrocotTreeNode {\n    fn partial_cmp(&self, other:\
    \ &Self) -> Option<std::cmp::Ordering> {\n        Some(self.cmp(other))\n    }\n\
    }\n\nimpl Ord for SternBrocotTreeNode {\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering\
    \ {\n        self.m.cmp(&other.m)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/rational/src/lib.rs
  isVerificationFile: false
  path: crates/math/stern-brocot-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-18 01:19:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stern_brocot_tree/src/main.rs
documentation_of: crates/math/stern-brocot-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/stern-brocot-tree/src/lib.rs
- /library/crates/math/stern-brocot-tree/src/lib.rs.html
title: crates/math/stern-brocot-tree/src/lib.rs
---
