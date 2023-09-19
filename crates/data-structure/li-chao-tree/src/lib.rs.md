---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/line_add_get_min_offline/src/main.rs
    title: verify/line_add_get_min_offline/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/segment_add_get_min_offline/src/main.rs
    title: verify/segment_add_get_min_offline/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.5/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::{\n    mem::swap,\n    ops::{Add, Bound, Mul, RangeBounds},\n};\n\
    \n#[derive(Clone, Copy)]\nstruct Line<T>(T, T)\nwhere\n    T: Copy + Add<Output\
    \ = T> + Mul<Output = T> + Ord + Default;\n\nimpl<T> Line<T>\nwhere\n    T: Copy\
    \ + Add<Output = T> + Mul<Output = T> + Ord + Default,\n{\n    fn eval(self, x:\
    \ T) -> T {\n        self.0 * x + self.1\n    }\n\n    fn over(self, other: Self,\
    \ x: T) -> bool {\n        self.eval(x) < other.eval(x)\n    }\n}\n\npub struct\
    \ LiChaoTree<T>\nwhere\n    T: Copy + Add<Output = T> + Mul<Output = T> + Ord\
    \ + Default,\n{\n    n: usize,\n    sz: usize,\n    xs: Vec<T>,\n    seg: Vec<Option<Line<T>>>,\n\
    }\n\nimpl<T> LiChaoTree<T>\nwhere\n    T: Copy + Add<Output = T> + Mul<Output\
    \ = T> + Ord + Default,\n{\n    pub fn new(mut xs: Vec<T>) -> Self {\n       \
    \ xs.sort();\n        xs.dedup();\n        if xs.len() == 0 {\n            xs.push(Default::default());\n\
    \        }\n        let sz = 1 << 64 - (xs.len().saturating_sub(1)).leading_zeros()\
    \ as usize;\n        let n = xs.len();\n        xs.resize(sz, *xs.last().unwrap());\n\
    \        Self {\n            n,\n            sz,\n            xs,\n          \
    \  seg: vec![None; sz * 2],\n        }\n    }\n\n    pub fn add_line(&mut self,\
    \ (a, b): (T, T)) {\n        self.update(Line(a, b), 0, self.sz, 1);\n    }\n\n\
    \    pub fn add_segment(&mut self, range: impl RangeBounds<T>, (a, b): (T, T))\
    \ {\n        let mut l = match range.start_bound() {\n            Bound::Unbounded\
    \ => 0,\n            Bound::Included(&l) => self.lower_bound(l),\n           \
    \ Bound::Excluded(&l) => self.upper_bound(l),\n        } + self.sz;\n        let\
    \ mut r = match range.end_bound() {\n            Bound::Unbounded => self.n,\n\
    \            Bound::Excluded(&r) => self.lower_bound(r),\n            Bound::Included(&r)\
    \ => self.upper_bound(r),\n        } + self.sz;\n        let line = Line(a, b);\n\
    \        while l < r {\n            if l & 1 != 0 {\n                self.update_(line,\
    \ l);\n                l += 1;\n            }\n            if r & 1 != 0 {\n \
    \               r -= 1;\n                self.update_(line, r);\n            }\n\
    \            l >>= 1;\n            r >>= 1;\n        }\n    }\n\n    pub fn min(&self,\
    \ x: T) -> Option<T> {\n        let k = self.xs[0..self.n].binary_search(&x).unwrap();\n\
    \        self.min_(x, k + self.sz)\n    }\n\n    fn min_(&self, x: T, mut k: usize)\
    \ -> Option<T> {\n        let mut res = None;\n        while k > 0 {\n       \
    \     res = min(res, self.seg[k].map(|o| o.eval(x)));\n            k >>= 1;\n\
    \        }\n        res\n    }\n\n    fn update(&mut self, mut line: Line<T>,\
    \ mut l: usize, mut r: usize, mut k: usize) {\n        loop {\n            let\
    \ m = (l + r) >> 1;\n            if let Some(o) = self.seg[k].as_mut() {\n   \
    \             let l_over = line.over(*o, self.xs[l]);\n                let r_over\
    \ = line.over(*o, self.xs[r - 1]);\n                if l_over == r_over {\n  \
    \                  if l_over {\n                        swap(o, &mut line);\n\
    \                    }\n                    return;\n                }\n     \
    \           let m_over = line.over(*o, self.xs[m]);\n                if m_over\
    \ {\n                    swap(o, &mut line);\n                }\n            \
    \    if l_over != m_over {\n                    k <<= 1;\n                   \
    \ r = m;\n                } else {\n                    k = k << 1 | 1;\n    \
    \                l = m;\n                }\n            } else {\n           \
    \     self.seg[k] = Some(line);\n                return;\n            }\n    \
    \    }\n    }\n\n    fn update_(&mut self, line: Line<T>, k: usize) {\n      \
    \  let u = 63 - k.leading_zeros();\n        let l = (self.sz >> u) * (k - (1 <<\
    \ u));\n        let r = l + (self.sz >> u);\n        self.update(line, l, r, k);\n\
    \    }\n\n    fn lower_bound(&self, x: T) -> usize {\n        match self.xs[0..self.n].binary_search(&x)\
    \ {\n            Ok(k) => k,\n            Err(k) => k,\n        }\n    }\n\n \
    \   fn upper_bound(&self, x: T) -> usize {\n        match self.xs[0..self.n].binary_search(&x)\
    \ {\n            Ok(k) => k + 1,\n            Err(k) => k,\n        }\n    }\n\
    }\n\nfn min<T>(x: Option<T>, y: Option<T>) -> Option<T>\nwhere\n    T: Copy +\
    \ Add<Output = T> + Mul<Output = T> + Ord,\n{\n    match (x, y) {\n        (Some(x),\
    \ Some(y)) => Some(x.min(y)),\n        (Some(x), None) => Some(x),\n        _\
    \ => y,\n    }\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/data-structure/li-chao-tree/src/lib.rs
  requiredBy: []
  timestamp: '2023-05-16 16:25:17+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/segment_add_get_min_offline/src/main.rs
  - verify/line_add_get_min_offline/src/main.rs
documentation_of: crates/data-structure/li-chao-tree/src/lib.rs
layout: document
redirect_from:
- /library/crates/data-structure/li-chao-tree/src/lib.rs
- /library/crates/data-structure/li-chao-tree/src/lib.rs.html
title: crates/data-structure/li-chao-tree/src/lib.rs
---
