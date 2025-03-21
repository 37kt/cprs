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
  _extendedRequiredBy:
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/wavelet_matrix/src/lib.rs
    title: crates/data_structure/wavelet_matrix/src/lib.rs
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
    title: verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
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
  code: "use numeric_traits::{Cast, Integer};\n\n/// SMALL == true: O(max-min) \u30B5\
    \u30A4\u30BA\u306E\u914D\u5217\u3092\u4F7F\u7528  \n/// SMALL == false: \u30BD\
    \u30FC\u30C8\u3057\u3066\u4E8C\u5206\u63A2\u7D22  \n/// DISTINCT == true: x \u305D\
    \u308C\u305E\u308C\u306B\u7570\u306A\u308B\u5024\u3092\u5272\u308A\u5F53\u3066\
    \u308B  \n/// DISTINCT == false: \u540C\u3058\u5024\u306B\u306F\u540C\u3058\u5024\
    \u3092\u5272\u308A\u5F53\u3066\u308B  \n#[derive(Clone)]\npub struct CoordinateCompression<T,\
    \ const SMALL: bool = false, const DISTINCT: bool = false>(\n    Container<T>,\n\
    )\nwhere\n    T: Integer + Cast<usize>,\n    u32: Cast<T>;\n\nimpl<T, const SMALL:\
    \ bool, const DISTINCT: bool> CoordinateCompression<T, SMALL, DISTINCT>\nwhere\n\
    \    T: Integer + Cast<usize>,\n    u32: Cast<T>,\n{\n    pub fn new(xs: impl\
    \ IntoIterator<Item = T>) -> (Self, Vec<usize>) {\n        let xs = xs.into_iter().collect::<Vec<_>>();\n\
    \        let n = xs.len();\n        if n == 0 {\n            return (Self(Container::Empty),\
    \ vec![]);\n        }\n        let min = *xs.iter().min().unwrap();\n        let\
    \ max = *xs.iter().max().unwrap();\n        let mut ys = vec![0; xs.len()];\n\
    \        if SMALL {\n            let len = (max - min).cast() + 1;\n         \
    \   let mut sum = vec![0; len + 1];\n            if DISTINCT {\n             \
    \   for &x in &xs {\n                    sum[(x - min).cast() + 1] += 1;\n   \
    \             }\n                for i in 0..len {\n                    sum[i\
    \ + 1] += sum[i];\n                }\n                for i in 0..n {\n      \
    \              let j = (xs[i] - min).cast();\n                    ys[i] = sum[j]\
    \ as usize;\n                    sum[j] += 1;\n                }\n           \
    \     for i in (1..=len).rev() {\n                    sum[i] = sum[i - 1];\n \
    \               }\n                sum[0] = 0;\n                return (Self(Container::Small(Small\
    \ { min, sum })), ys);\n            } else {\n                for &x in &xs {\n\
    \                    sum[(x - min).cast() + 1] = 1;\n                }\n     \
    \           for i in 0..len {\n                    sum[i + 1] += sum[i];\n   \
    \             }\n                for i in 0..n {\n                    let j =\
    \ (xs[i] - min).cast();\n                    ys[i] = sum[j] as usize;\n      \
    \          }\n                return (Self(Container::Small(Small { min, sum })),\
    \ ys);\n            }\n        } else {\n            if DISTINCT {\n         \
    \       let mut ord = (0..n).collect::<Vec<_>>();\n                ord.sort_by_key(|&i|\
    \ xs[i]);\n                let mut new_xs = Vec::with_capacity(n);\n         \
    \       for i in 0..n {\n                    ys[ord[i]] = i;\n               \
    \     new_xs.push(xs[ord[i]]);\n                }\n                return (Self(Container::Large(Large\
    \ { xs: new_xs })), ys);\n            } else {\n                let mut ord =\
    \ (0..n).collect::<Vec<_>>();\n                ord.sort_unstable_by_key(|&i| xs[i]);\n\
    \                let mut new_xs = Vec::with_capacity(n);\n                for\
    \ i in 0..n {\n                    if i > 0 && xs[ord[i]] == xs[ord[i - 1]] {\n\
    \                        ys[ord[i]] = new_xs.len() - 1;\n                    }\
    \ else {\n                        ys[ord[i]] = new_xs.len();\n               \
    \         new_xs.push(xs[ord[i]]);\n                    }\n                }\n\
    \                new_xs.shrink_to_fit();\n                return (Self(Container::Large(Large\
    \ { xs: new_xs })), ys);\n            }\n        }\n    }\n\n    pub fn encode(&self,\
    \ x: T) -> usize {\n        match &self.0 {\n            Container::Empty => 0,\n\
    \            Container::Small(Small { min, sum, .. }) => {\n                let\
    \ j = (x - *min).max(T::zero()).cast().min(sum.len() - 1);\n                sum[j]\
    \ as usize\n            }\n            Container::Large(Large { xs }) => xs.partition_point(|&xi|\
    \ xi < x),\n        }\n    }\n\n    pub fn decode(&self, i: usize) -> T {\n  \
    \      match &self.0 {\n            Container::Empty => panic!(\"out of range\"\
    ),\n            Container::Small(Small { min, sum, .. }) => *min + sum[i].cast(),\n\
    \            Container::Large(Large { xs }) => xs[i],\n        }\n    }\n}\n\n\
    impl<T, const SMALL: bool, const DISTINCT: bool> CoordinateCompression<T, SMALL,\
    \ DISTINCT>\nwhere\n    T: Integer + Cast<usize>,\n    u32: Cast<T>,\n{\n    ///\
    \ \u5024\u306E\u7BC4\u56F2 \\[0, n) \u306E n \u3092\u8FD4\u3059\n    pub fn len(&self)\
    \ -> usize {\n        match &self.0 {\n            Container::Empty => 0,\n  \
    \          Container::Small(Small { sum, .. }) => *sum.last().unwrap() as usize,\n\
    \            Container::Large(Large { xs }) => xs.len(),\n        }\n    }\n}\n\
    \n#[derive(Clone)]\nstruct Small<T> {\n    min: T,\n    sum: Vec<u32>,\n}\n\n\
    #[derive(Clone)]\nstruct Large<T> {\n    xs: Vec<T>,\n}\n\n#[derive(Clone)]\n\
    enum Container<T> {\n    Empty,\n    Small(Small<T>),\n    Large(Large<T>),\n\
    }\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  isVerificationFile: false
  path: crates/misc/coordinate_compression/src/lib.rs
  requiredBy:
  - crates/data_structure/wavelet_matrix/src/lib.rs
  timestamp: '2025-03-20 09:27:03+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/data_structure/static_range_frequency_mo/src/main.rs
documentation_of: crates/misc/coordinate_compression/src/lib.rs
layout: document
redirect_from:
- /library/crates/misc/coordinate_compression/src/lib.rs
- /library/crates/misc/coordinate_compression/src/lib.rs.html
title: crates/misc/coordinate_compression/src/lib.rs
---
