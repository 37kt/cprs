---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/predecessor_problem/src/main.rs
    title: verify/predecessor_problem/src/main.rs
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
  code: "#[derive(Clone)]\npub struct WAryTreeSet {\n    m: usize,\n    v: Box<[Box<[usize]>]>,\n\
    }\n\nimpl WAryTreeSet {\n    pub fn new(mut n: usize) -> Self {\n        let mut\
    \ v = vec![];\n        let mut l = 1;\n        while n > 0 {\n            n >>=\
    \ 6;\n            v.push(vec![0; l].into_boxed_slice());\n            l <<= 6;\n\
    \        }\n        Self {\n            m: v.len(),\n            v: v.into_boxed_slice(),\n\
    \        }\n    }\n\n    pub fn contains(&self, x: usize) -> bool {\n        self.v[self.m\
    \ - 1][x >> 6] & 1 << (x & 63) != 0\n    }\n\n    pub fn insert(&mut self, mut\
    \ x: usize) -> bool {\n        if self.contains(x) {\n            false\n    \
    \    } else {\n            for v in self.v.iter_mut().rev() {\n              \
    \  v[x >> 6] |= 1 << (x & 63);\n                x >>= 6;\n            }\n    \
    \        true\n        }\n    }\n\n    pub fn remove(&mut self, mut x: usize)\
    \ -> bool {\n        if !self.contains(x) {\n            false\n        } else\
    \ {\n            for i in (0..self.m).rev() {\n                if i + 1 == self.m\
    \ || self.v[i + 1][x] == 0 {\n                    self.v[i][x >> 6] ^= 1 << (x\
    \ & 63);\n                }\n                x >>= 6;\n            }\n       \
    \     true\n        }\n    }\n\n    pub fn min(&self) -> Option<usize> {\n   \
    \     (self.v[0][0] != 0).then(|| {\n            self.v\n                .iter()\n\
    \                .fold(0, |t, v| t << 6 | v[t].trailing_zeros() as usize)\n  \
    \      })\n    }\n\n    pub fn max(&self) -> Option<usize> {\n        (self.v[0][0]\
    \ != 0).then(|| {\n            self.v\n                .iter()\n             \
    \   .fold(0, |t, v| t << 6 | 63 - v[t].leading_zeros() as usize)\n        })\n\
    \    }\n\n    pub fn next(&self, mut x: usize) -> Option<usize> {\n        for\
    \ i in (0..self.m).rev() {\n            let mask = if i + 1 == self.m {\n    \
    \            !((1 << (x & 63)) - 1)\n            } else {\n                !0\
    \ << (x & 63) << 1\n            };\n            let t = (self.v[i][x >> 6] & mask).trailing_zeros()\
    \ as usize;\n            if t != 64 {\n                let mut t = x & !63 | t;\n\
    \                for j in i + 1..self.m {\n                    t = t << 6 | self.v[j][t].trailing_zeros()\
    \ as usize;\n                }\n                return Some(t);\n            }\n\
    \            x >>= 6;\n        }\n        None\n    }\n\n    pub fn prev(&self,\
    \ mut x: usize) -> Option<usize> {\n        for i in (0..self.m).rev() {\n   \
    \         let mask = if i + 1 == self.m {\n                !(!0 << (x & 63) <<\
    \ 1)\n            } else {\n                (1 << (x & 63)) - 1\n            };\n\
    \            let t = (self.v[i][x >> 6] & mask).leading_zeros() as usize;\n  \
    \          if t != 64 {\n                let mut t = x >> 6 << 6 | 63 - t;\n \
    \               for j in i + 1..self.m {\n                    t = t << 6 | 63\
    \ - self.v[j][t].leading_zeros() as usize;\n                }\n              \
    \  return Some(t);\n            }\n            x >>= 6;\n        }\n        None\n\
    \    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/w-ary-tree-set/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-13 18:34:42+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/predecessor_problem/src/main.rs
documentation_of: crates/data-structure/w-ary-tree-set/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/w-ary-tree-set/src/lib.rs
- /library/crates/data-structure/w-ary-tree-set/src/lib.rs.html
title: crates/data-structure/w-ary-tree-set/src/lib.rs
---
