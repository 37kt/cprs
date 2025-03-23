---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/cast.rs
    title: crates/algebra/numeric_traits/src/cast.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/inf.rs
    title: crates/algebra/numeric_traits/src/inf.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/integer.rs
    title: crates/algebra/numeric_traits/src/integer.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/lib.rs
    title: crates/algebra/numeric_traits/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/numeric.rs
    title: crates/algebra/numeric_traits/src/numeric.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/signed.rs
    title: crates/algebra/numeric_traits/src/signed.rs
  - icon: ':heavy_check_mark:'
    path: crates/algebra/numeric_traits/src/zero_one.rs
    title: crates/algebra/numeric_traits/src/zero_one.rs
  - icon: ':warning:'
    path: crates/misc/into_half_open_range/src/lib.rs
    title: crates/misc/into_half_open_range/src/lib.rs
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/string/suffix_array/src/lib.rs
    title: crates/string/suffix_array/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/staticrmq/src/main.rs
    title: verify/library_checker/data_structure/staticrmq/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "use std::ops::RangeBounds;\n\nuse into_half_open_range::IntoHalfOpenRange;\n\
    use numeric_traits::Integer;\n\nconst BLOCK_SIZE: usize = 16;\n\n#[derive(Clone)]\n\
    pub struct RangeMinimumQuery<T> {\n    array: Vec<T>,\n    large: Vec<Vec<u32>>,\n\
    \    small: Vec<u16>,\n}\n\nimpl<T> FromIterator<T> for RangeMinimumQuery<T>\n\
    where\n    T: Ord,\n{\n    fn from_iter<I: IntoIterator<Item = T>>(iter: I) ->\
    \ Self {\n        let xs = iter.into_iter().collect::<Vec<_>>();\n        let\
    \ n = xs.len();\n        let block_n = n.floor_div(BLOCK_SIZE);\n        let large\
    \ = if let Some(lg_block_n) = block_n.checked_floor_log2() {\n            let\
    \ mut large = Vec::with_capacity(lg_block_n + 1);\n            let level = xs\n\
    \                .chunks_exact(BLOCK_SIZE)\n                .enumerate()\n   \
    \             .map(|(b, v)| {\n                    (v.iter()\n               \
    \         .enumerate()\n                        .min_by_key(|&(_, x)| x)\n   \
    \                     .map(|(i, _)| i)\n                        .unwrap()\n  \
    \                      + b * BLOCK_SIZE) as u32\n                })\n        \
    \        .collect::<Vec<_>>();\n            large.push(level);\n            for\
    \ i in 0..lg_block_n {\n                let level = large[i]\n               \
    \     .iter()\n                    .zip(&large[i][1 << i..])\n               \
    \     .map(|(&a, &b)| {\n                        if xs[a as usize] <= xs[b as\
    \ usize] {\n                            a\n                        } else {\n\
    \                            b\n                        }\n                  \
    \  })\n                    .collect::<Vec<_>>();\n                large.push(level);\n\
    \            }\n            large\n        } else {\n            vec![]\n    \
    \    };\n        let mut small = Vec::with_capacity(n);\n        let mut stack\
    \ = Vec::<usize>::with_capacity(BLOCK_SIZE);\n        let mut bit = 0;\n     \
    \   for i in 0..n {\n            if i % BLOCK_SIZE == 0 {\n                stack.clear();\n\
    \                bit = 0;\n            }\n            while let Some(&j) = stack.last()\
    \ {\n                if xs[j] <= xs[i] {\n                    break;\n       \
    \         }\n                bit &= !(1 << j % BLOCK_SIZE);\n                stack.pop();\n\
    \            }\n            stack.push(i);\n            bit |= 1 << i % BLOCK_SIZE;\n\
    \            small.push(bit);\n        }\n\n        Self {\n            array:\
    \ xs,\n            large,\n            small,\n        }\n    }\n}\n\nimpl<T>\
    \ RangeMinimumQuery<T>\nwhere\n    T: Ord,\n{\n    pub fn min(&self, range: impl\
    \ RangeBounds<usize>) -> Option<&T> {\n        let (l, r) = range.into_half_open_range(0,\
    \ self.array.len());\n        if l == r {\n            return None;\n        }\n\
    \        let bl = l.ceil_div(BLOCK_SIZE);\n        let br = r.floor_div(BLOCK_SIZE);\n\
    \        let mut res = None;\n        if bl > br {\n            let i = (self.small[r\
    \ - 1] & (!0 << l % BLOCK_SIZE)).lsb_index();\n            res = Self::merge(res,\
    \ Some(&self.array[i + br * BLOCK_SIZE]));\n        } else {\n            if bl\
    \ < br {\n                let d = (br - bl).floor_log2();\n                let\
    \ level = &self.large[d];\n                res = Self::merge(res, Some(&self.array[level[bl]\
    \ as usize]));\n                res = Self::merge(res, Some(&self.array[level[br\
    \ - (1 << d)] as usize]));\n            }\n            if l % BLOCK_SIZE != 0\
    \ {\n                let i = (self.small[bl * BLOCK_SIZE - 1] & (!0 << l % BLOCK_SIZE)).lsb_index();\n\
    \                res = Self::merge(res, Some(&self.array[i + (bl - 1) * BLOCK_SIZE]));\n\
    \            }\n            if r % BLOCK_SIZE != 0 {\n                let i =\
    \ self.small[r - 1].lsb_index();\n                res = Self::merge(res, Some(&self.array[i\
    \ + br * BLOCK_SIZE]));\n            }\n        }\n        res\n    }\n\n    fn\
    \ merge<'a>(x: Option<&'a T>, y: Option<&'a T>) -> Option<&'a T> {\n        x.into_iter().chain(y.into_iter()).min()\n\
    \    }\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/misc/into_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/data_structure/range_minimum_query/src/lib.rs
  requiredBy:
  - crates/string/suffix_array/src/lib.rs
  timestamp: '2025-03-23 00:32:36+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/staticrmq/src/main.rs
documentation_of: crates/data_structure/range_minimum_query/src/lib.rs
layout: document
redirect_from:
- /library/crates/data_structure/range_minimum_query/src/lib.rs
- /library/crates/data_structure/range_minimum_query/src/lib.rs.html
title: crates/data_structure/range_minimum_query/src/lib.rs
---
