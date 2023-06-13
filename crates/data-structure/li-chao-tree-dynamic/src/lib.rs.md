---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/line_add_get_min/src/main.rs
    title: verify/line_add_get_min/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/segment_add_get_min/src/main.rs
    title: verify/segment_add_get_min/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.3/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "fn mid(l: i64, r: i64) -> i64 {\n    l + (r - l) / 2\n}\n\nfn min(a: Option<i64>,\
    \ b: Option<i64>) -> Option<i64> {\n    match (a, b) {\n        (None, _) => b,\n\
    \        (_, None) => a,\n        (Some(a), Some(b)) => Some(a.min(b)),\n    }\n\
    }\n\n#[derive(Clone, Copy)]\nstruct Line(i64, i64);\n\nimpl Line {\n    fn eval(&self,\
    \ x: i64) -> i64 {\n        self.0 * x + self.1\n    }\n}\n\nstruct Node {\n \
    \   f: Option<Line>,\n    l: Option<Box<Node>>,\n    r: Option<Box<Node>>,\n}\n\
    \nimpl Node {\n    fn find(&self, l: i64, r: i64, x: i64) -> Option<i64> {\n \
    \       assert!(l <= x && x < r);\n        let mut res = None;\n        if let\
    \ Some(f) = self.f {\n            let y = f.eval(x);\n            res = min(res,\
    \ Some(y));\n        }\n        let m = mid(l, r);\n        if x < m {\n     \
    \       if let Some(c) = &self.l {\n                let y = c.find(l, m, x);\n\
    \                res = min(res, y);\n            }\n        } else {\n       \
    \     if let Some(c) = &self.r {\n                let y = c.find(m, r, x);\n \
    \               res = min(res, y);\n            }\n        }\n        res\n  \
    \  }\n\n    fn insert(&mut self, l: i64, r: i64, a: i64, b: i64, mut f: Line)\
    \ {\n        if r <= a || b <= l {\n            return;\n        }\n        if\
    \ a <= l && r <= b {\n            if self.f.is_none() {\n                self.f\
    \ = Some(f);\n                return;\n            }\n            if self.f.unwrap().eval(l)\
    \ > f.eval(l) {\n                std::mem::swap(self.f.as_mut().unwrap(), &mut\
    \ f);\n            }\n            let m = mid(l, r);\n            if self.f.unwrap().eval(r\
    \ - 1) <= f.eval(r - 1) {\n                return;\n            } else if self.f.unwrap().eval(m)\
    \ < f.eval(m) {\n                if self.r.is_none() {\n                    self.r\
    \ = Some(Box::new(Node {\n                        f: None,\n                 \
    \       l: None,\n                        r: None,\n                    }));\n\
    \                }\n                self.r.as_mut().unwrap().insert(m, r, a, b,\
    \ f);\n            } else {\n                if self.l.is_none() {\n         \
    \           self.l = Some(Box::new(Node {\n                        f: None,\n\
    \                        l: None,\n                        r: None,\n        \
    \            }));\n                }\n                std::mem::swap(self.f.as_mut().unwrap(),\
    \ &mut f);\n                self.l.as_mut().unwrap().insert(l, m, a, b, f);\n\
    \            }\n        } else {\n            let m = mid(l, r);\n           \
    \ if self.l.is_none() {\n                self.l = Some(Box::new(Node {\n     \
    \               f: None,\n                    l: None,\n                    r:\
    \ None,\n                }));\n            }\n            self.l.as_mut().unwrap().insert(l,\
    \ m, a, b, f);\n            if self.r.is_none() {\n                self.r = Some(Box::new(Node\
    \ {\n                    f: None,\n                    l: None,\n            \
    \        r: None,\n                }));\n            }\n            self.r.as_mut().unwrap().insert(m,\
    \ r, a, b, f);\n        }\n    }\n}\n\npub struct LiChaoTreeDynamic {\n    root:\
    \ Option<Node>,\n    min_x: i64,\n    max_x: i64,\n    max_query: bool,\n}\n\n\
    impl LiChaoTreeDynamic {\n    // max_query\u304Cfalse\u306A\u3089\u6700\u5C0F\u5024\
    \u30AF\u30A8\u30EA\u3001true\u306A\u3089\u6700\u5927\u5024\u30AF\u30A8\u30EA\n\
    \    pub fn new(min_x: i64, max_x: i64, max_query: bool) -> Self {\n        Self\
    \ {\n            root: None,\n            min_x,\n            max_x,\n       \
    \     max_query,\n        }\n    }\n\n    // \u76F4\u7DDAax+b\u3092\u8FFD\u52A0\
    \n    pub fn add_line(&mut self, a: i64, b: i64) {\n        self.add_segment(a,\
    \ b, self.min_x, self.max_x);\n    }\n\n    // \u7DDA\u5206ax+b(l<=x<r)\u3092\u8FFD\
    \u52A0\n    pub fn add_segment(&mut self, mut a: i64, mut b: i64, l: i64, r: i64)\
    \ {\n        if self.max_query {\n            a *= -1;\n            b *= -1;\n\
    \        }\n        if self.root.is_none() {\n            self.root = Some(Node\
    \ {\n                f: None,\n                l: None,\n                r: None,\n\
    \            });\n        }\n        self.root\n            .as_mut()\n      \
    \      .unwrap()\n            .insert(self.min_x, self.max_x, l, r, Line(a, b));\n\
    \    }\n\n    // ax+b\u306Ex\u3092\u5B9A\u3081\u305F\u3068\u304D\u306E\u6700\u5C0F\
    \u5024\u3092\u6C42\u3081\u308B\n    pub fn find(&self, x: i64) -> Option<i64>\
    \ {\n        self.root\n            .as_ref()\n            .and_then(|v| v.find(self.min_x,\
    \ self.max_x, x))\n            .map(|res| res * if self.max_query { -1 } else\
    \ { 1 })\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/li-chao-tree-dynamic/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-16 16:25:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/segment_add_get_min/src/main.rs
  - verify/line_add_get_min/src/main.rs
documentation_of: crates/data-structure/li-chao-tree-dynamic/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/li-chao-tree-dynamic/src/lib.rs
- /library/crates/data-structure/li-chao-tree-dynamic/src/lib.rs.html
title: crates/data-structure/li-chao-tree-dynamic/src/lib.rs
---
