---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/rational/src/lib.rs
    title: crates/algebraic/rational/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/rational_approximation/src/main.rs
    title: verify/rational_approximation/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/stern_brocot_tree/src/main.rs
    title: verify/stern_brocot_tree/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.4/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use rational::{Rational, ZTrait};\n\npub struct SternBrocotTreeNode<T>\n\
    where\n    T: ZTrait,\n{\n    l: Rational<T>,\n    m: Rational<T>,\n    r: Rational<T>,\n\
    \    seq: Vec<T>,\n}\n\nimpl<T> Default for SternBrocotTreeNode<T>\nwhere\n  \
    \  T: ZTrait,\n{\n    fn default() -> Self {\n        Self {\n            l: Rational::new(T::zero(),\
    \ T::one()),\n            m: Rational::new(T::one(), T::one()),\n            r:\
    \ Rational::new(T::one(), T::zero()),\n            seq: vec![],\n        }\n \
    \   }\n}\n\nimpl<T> SternBrocotTreeNode<T>\nwhere\n    T: ZTrait,\n{\n    pub\
    \ fn new(mut x: Rational<T>) -> Self {\n        assert!(x.num > T::zero() && x.den\
    \ > T::zero());\n        x.reduce();\n        let mut n = Self::default();\n \
    \       while x.num.min(x.den) > T::zero() {\n            if x.num > x.den {\n\
    \                let d = x.num / x.den;\n                x.num -= x.den * d;\n\
    \                n.go_child(\n                    d - if x.num == T::zero() {\n\
    \                        T::one()\n                    } else {\n            \
    \            T::zero()\n                    },\n                );\n         \
    \   } else {\n                let d = x.den / x.num;\n                x.den -=\
    \ x.num * d;\n                n.go_child(\n                    if x.den == T::zero()\
    \ {\n                        T::one()\n                    } else {\n        \
    \                T::zero()\n                    } - d,\n                );\n \
    \           }\n        }\n        n\n    }\n\n    pub fn depth(&self) -> T {\n\
    \        self.seq.iter().map(|&x| x.abs()).sum::<T>()\n    }\n\n    pub fn go_child(&mut\
    \ self, d: T) {\n        if d == T::zero() {\n            return;\n        }\n\
    \        if self.seq.is_empty() || (*self.seq.last().unwrap() < T::zero()) !=\
    \ (d < T::zero()) {\n            self.seq.push(d);\n        } else {\n       \
    \     *self.seq.last_mut().unwrap() += d;\n        }\n        if d < T::zero()\
    \ {\n            self.r.num += self.l.num * -d;\n            self.r.den += self.l.den\
    \ * -d;\n            self.m = Rational {\n                num: self.l.num + self.r.num,\n\
    \                den: self.l.den + self.r.den,\n            };\n        } else\
    \ {\n            self.l.num += self.r.num * d;\n            self.l.den += self.r.den\
    \ * d;\n            self.m = Rational {\n                num: self.l.num + self.r.num,\n\
    \                den: self.l.den + self.r.den,\n            };\n        }\n  \
    \  }\n\n    pub fn go_parent(&mut self, mut d: T) -> bool {\n        assert!(d\
    \ >= T::zero());\n        if d == T::zero() {\n            return true;\n    \
    \    }\n        while d != T::zero() {\n            if self.seq.is_empty() {\n\
    \                return false;\n            }\n            let mut x = self.seq.pop().unwrap();\n\
    \            let d2 = d.min(x.abs());\n            if x < T::zero() {\n      \
    \          self.m.num -= self.l.num * d2;\n                self.m.den -= self.l.den\
    \ * d2;\n                self.r = Rational {\n                    num: self.m.num\
    \ - self.l.num,\n                    den: self.m.den - self.l.den,\n         \
    \       };\n                x += d2;\n            } else {\n                self.m.num\
    \ -= self.r.num * d2;\n                self.m.den -= self.r.den * d2;\n      \
    \          self.l = Rational {\n                    num: self.m.num - self.r.num,\n\
    \                    den: self.m.den - self.r.den,\n                };\n     \
    \           x -= d2;\n            }\n            d -= d2;\n            if x !=\
    \ T::zero() {\n                self.seq.push(x);\n            }\n            if\
    \ d2 == T::zero() {\n                break;\n            }\n        }\n      \
    \  true\n    }\n\n    pub fn lca(&self, other: &SternBrocotTreeNode<T>) -> SternBrocotTreeNode<T>\
    \ {\n        let mut res = SternBrocotTreeNode::default();\n        for (&x, &y)\
    \ in self.seq.iter().zip(&other.seq) {\n            if (x < T::zero()) != (y <\
    \ T::zero()) {\n                break;\n            }\n            if x < T::zero()\
    \ {\n                res.go_child(x.max(y));\n            } else if x > T::zero()\
    \ {\n                res.go_child(x.min(y));\n            }\n            if x\
    \ != y {\n                break;\n            }\n        }\n        res\n    }\n\
    \n    pub fn lower_bound(&self) -> Rational<T> {\n        self.l\n    }\n\n  \
    \  pub fn upper_bound(&self) -> Rational<T> {\n        self.r\n    }\n\n    pub\
    \ fn val(&self) -> Rational<T> {\n        self.m\n    }\n\n    pub fn path(&self)\
    \ -> Vec<T> {\n        self.seq.clone()\n    }\n\n    pub fn binary_search(f:\
    \ impl Fn(Rational<T>) -> bool, lim: T) -> (Rational<T>, Rational<T>) {\n    \
    \    let mut l = Rational::new(T::zero(), T::one());\n        let mut r = Rational::new(T::one(),\
    \ T::zero());\n        let f01 = f(l);\n        let f10 = f(r);\n        assert!(f01\
    \ != f10);\n        loop {\n            let m = Rational {\n                num:\
    \ l.num + r.num,\n                den: l.den + r.den,\n            };\n      \
    \      if m.num > lim || m.den > lim {\n                break;\n            }\n\
    \            let fm = f(m);\n            let sgn = (if fm != f01 { -1 } else {\
    \ 1 }).into();\n            let maxd = if sgn == -T::one() {\n               \
    \ if l.num == T::zero() {\n                    (lim - r.den) / l.den\n       \
    \         } else {\n                    ((lim - r.den) / l.den).min((lim - r.num)\
    \ / l.num)\n                }\n            } else {\n                if r.den\
    \ == T::zero() {\n                    (lim - l.num) / r.num\n                }\
    \ else {\n                    ((lim - l.num) / r.num).min((lim - l.den) / r.den)\n\
    \                }\n            };\n            let mut dl: T = 1.into();\n  \
    \          let mut dr: T = 2.into();\n            while dr <= maxd {\n       \
    \         let mut n = SternBrocotTreeNode::new(m);\n                n.go_child(dr\
    \ * sgn);\n                if f(n.val()) == fm {\n                    dl = dr;\n\
    \                    dr = (dr * 2.into()).min(maxd + 1.into());\n            \
    \    } else {\n                    break;\n                }\n            }\n\
    \            while dl + 1.into() < dr {\n                let dm = dl + (dr - dl)\
    \ / 2.into();\n                let mut n = SternBrocotTreeNode::new(m);\n    \
    \            n.go_child(dm * sgn);\n                if f(n.val()) == fm {\n  \
    \                  dl = dm;\n                } else {\n                    dr\
    \ = dm;\n                }\n            }\n            let mut n = SternBrocotTreeNode::new(m);\n\
    \            n.go_child(dl * sgn);\n            l = n.lower_bound();\n       \
    \     r = n.upper_bound();\n        }\n        (l, r)\n    }\n}\n\nimpl<T> PartialEq\
    \ for SternBrocotTreeNode<T>\nwhere\n    T: ZTrait,\n{\n    fn eq(&self, other:\
    \ &Self) -> bool {\n        self.m == other.m\n    }\n}\n\nimpl<T> Eq for SternBrocotTreeNode<T>\
    \ where T: ZTrait {}\n\nimpl<T> PartialOrd for SternBrocotTreeNode<T>\nwhere\n\
    \    T: ZTrait,\n{\n    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>\
    \ {\n        Some(self.cmp(other))\n    }\n}\n\nimpl<T> Ord for SternBrocotTreeNode<T>\n\
    where\n    T: ZTrait,\n{\n    fn cmp(&self, other: &Self) -> std::cmp::Ordering\
    \ {\n        self.m.cmp(&other.m)\n    }\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/algebraic/rational/src/lib.rs
  isVerificationFile: false
  path: crates/math/stern-brocot-tree/src/lib.rs
  requiredBy: []
  timestamp: '2024-03-21 11:10:47+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/stern_brocot_tree/src/main.rs
  - verify/rational_approximation/src/main.rs
documentation_of: crates/math/stern-brocot-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/stern-brocot-tree/src/lib.rs
- /library/crates/math/stern-brocot-tree/src/lib.rs.html
title: crates/math/stern-brocot-tree/src/lib.rs
---
