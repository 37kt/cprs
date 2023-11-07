---
data:
  _extendedDependsOn: []
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/rolling-hash/src/lib.rs
    title: crates/string/rolling-hash/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/nim_product_64/src/main.rs
    title: verify/nim_product_64/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.12.0/x64/lib/python3.12/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.12.0/x64/lib/python3.12/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    convert::TryInto,\n    fmt::{Debug, Display},\n    mem::swap,\n\
    \    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},\n    str::FromStr,\n\
    };\n\n#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]\npub struct Nimber(usize);\n\
    \nimpl FromStr for Nimber {\n    type Err = <usize as FromStr>::Err;\n    fn from_str(s:\
    \ &str) -> Result<Self, Self::Err> {\n        Ok(Nimber(s.parse::<usize>()?))\n\
    \    }\n}\n\nimpl<T, E> From<T> for Nimber\nwhere\n    T: TryInto<usize, Error\
    \ = E>,\n    E: Debug,\n{\n    fn from(x: T) -> Self {\n        Nimber(x.try_into().unwrap())\n\
    \    }\n}\n\nimpl Nimber {\n    pub fn new<T, E>(x: T) -> Self\n    where\n  \
    \      T: TryInto<usize, Error = E>,\n        E: Debug,\n    {\n        Self::from(x)\n\
    \    }\n\n    pub fn val(&self) -> usize {\n        self.0\n    }\n}\n\nimpl Display\
    \ for Nimber {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result\
    \ {\n        write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Debug for Nimber {\n\
    \    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n  \
    \      write!(f, \"{}\", self.0)\n    }\n}\n\nimpl Neg for Nimber {\n    type\
    \ Output = Self;\n    fn neg(self) -> Self::Output {\n        self\n    }\n}\n\
    \nimpl Neg for &Nimber {\n    type Output = Nimber;\n    fn neg(self) -> Self::Output\
    \ {\n        *self\n    }\n}\n\nimpl AddAssign for Nimber {\n    fn add_assign(&mut\
    \ self, rhs: Self) {\n        self.0 ^= rhs.0;\n    }\n}\n\nimpl SubAssign for\
    \ Nimber {\n    fn sub_assign(&mut self, rhs: Self) {\n        self.0 ^= rhs.0;\n\
    \    }\n}\n\nimpl MulAssign for Nimber {\n    fn mul_assign(&mut self, rhs: Self)\
    \ {\n        PRECALC.with(|precalc| {\n            SMALL.with(|small| {\n    \
    \            let mut res = 0;\n                for d in 0..8 {\n             \
    \       for e in 0..8 {\n                        res ^= precalc[d][e][small[self.0\
    \ >> d * 8 & 255][rhs.0 >> e * 8 & 255]];\n                    }\n           \
    \     }\n                self.0 = res;\n            })\n        })\n    }\n}\n\
    \nmacro_rules! impl_ops {\n    ($(\n        $trait:ident,\n        $trait_assign:ident,\n\
    \        $fn:ident,\n        $fn_assign:ident,\n    )*) => {$(\n        impl $trait_assign<&Nimber>\
    \ for Nimber {\n            fn $fn_assign(&mut self, rhs: &Nimber) {\n       \
    \         self.$fn_assign(*rhs);\n            }\n        }\n        impl $trait<&Nimber>\
    \ for Nimber {\n            type Output = Nimber;\n            fn $fn(mut self,\
    \ rhs: &Nimber) -> Self::Output {\n                self.$fn_assign(*rhs);\n  \
    \              self\n            }\n        }\n        impl $trait<Nimber> for\
    \ &Nimber {\n            type Output = Nimber;\n            fn $fn(self, rhs:\
    \ Nimber) -> Self::Output {\n                (*self).$fn(rhs)\n            }\n\
    \        }\n        impl $trait<Nimber> for Nimber {\n            type Output\
    \ = Nimber;\n            fn $fn(mut self, rhs: Nimber) -> Self::Output {\n   \
    \             self.$fn_assign(rhs);\n                self\n            }\n   \
    \     }\n        impl $trait<&Nimber> for &Nimber {\n            type Output =\
    \ Nimber;\n            fn $fn(self, rhs: &Nimber) -> Self::Output {\n        \
    \        (*self).$fn(&*rhs)\n            }\n        }\n    )*};\n}\n\nimpl_ops!\
    \ {\n    Add, AddAssign, add, add_assign,\n    Sub, SubAssign, sub, sub_assign,\n\
    \    Mul, MulAssign, mul, mul_assign,\n}\n\nfn rec(mut x: usize, mut y: usize)\
    \ -> usize {\n    if x == 0 || y == 0 {\n        return 0;\n    }\n    if x <\
    \ y {\n        swap(&mut x, &mut y);\n    }\n    if y == 1 {\n        return x;\n\
    \    }\n    for k in (0..6).rev() {\n        let shift = 1 << k;\n        let\
    \ mask = (1 << shift) - 1;\n        if y >> shift != 0 {\n            let v00\
    \ = rec(x & mask, y & mask);\n            let v01 = rec(x & mask, y >> shift);\n\
    \            let v10 = rec(x >> shift, y & mask);\n            let v11 = rec(x\
    \ >> shift, y >> shift);\n            return v00 ^ (v01 ^ v10 ^ v11) << shift\
    \ ^ rec(v11, 1 << shift - 1);\n        } else if x >> shift != 0 {\n         \
    \   return rec(x >> shift, y) << shift ^ rec(x & mask, y);\n        }\n    }\n\
    \    unreachable!()\n}\n\nfn small() -> Box<[Box<[usize]>]> {\n    let mut t =\
    \ vec![vec![0; 256].into_boxed_slice(); 256].into_boxed_slice();\n    for i in\
    \ 0..256 {\n        for j in 0..256 {\n            t[i][j] = rec(i, j);\n    \
    \    }\n    }\n    t\n}\n\nfn precalc() -> Box<[Box<[Box<[usize]>]>]> {\n    let\
    \ mut t =\n        vec![vec![vec![0; 256].into_boxed_slice(); 8].into_boxed_slice();\
    \ 8].into_boxed_slice();\n    for d in 0..8 {\n        for e in 0..8 {\n     \
    \       let p = rec(1 << d * 8, 1 << e * 8);\n            for i in 0..256 {\n\
    \                t[d][e][i] = rec(p, i);\n            }\n        }\n    }\n  \
    \  t\n}\n\nthread_local! {\n    static SMALL: Box<[Box<[usize]>]> = small();\n\
    \    static PRECALC: Box<[Box<[Box<[usize]>]>]> = precalc();\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/math/nimber/src/lib.rs
  requiredBy:
  - crates/string/rolling-hash/src/lib.rs
  timestamp: '2023-05-19 16:25:08+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/nim_product_64/src/main.rs
documentation_of: crates/math/nimber/src/lib.rs
layout: document
redirect_from:
- /library/crates/math/nimber/src/lib.rs
- /library/crates/math/nimber/src/lib.rs.html
title: crates/math/nimber/src/lib.rs
---
