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
  - icon: ':heavy_check_mark:'
    path: crates/data_structure/range_minimum_query/src/lib.rs
    title: crates/data_structure/range_minimum_query/src/lib.rs
  - icon: ':warning:'
    path: crates/misc/into_half_open_range/src/lib.rs
    title: crates/misc/into_half_open_range/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/library_checker/string/suffixarray/src/main.rs
    title: verify/library_checker/string/suffixarray/src/main.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links:
    - https://rsk0315.github.io/library-rs/nekolib/seq/suffix_array/struct.SuffixArray.html#fnref2
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.13.2/x64/lib/python3.13/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// https://rsk0315.github.io/library-rs/nekolib/seq/suffix_array/struct.SuffixArray.html#fnref2\n\
    \nuse std::{cmp::Ordering, ops::RangeBounds};\n\nuse into_half_open_range::IntoHalfOpenRange;\n\
    use numeric_traits::{Cast, Integer};\nuse range_minimum_query::RangeMinimumQuery;\n\
    \n#[derive(Clone)]\npub struct SuffixArray {\n    sa: Vec<usize>,\n    sa_inv:\
    \ Vec<usize>,\n    lcp: Vec<usize>,\n    rmq: RangeMinimumQuery<usize>,\n}\n\n\
    impl SuffixArray {\n    pub fn new<T>(s: &[T]) -> Self\n    where\n        T:\
    \ Integer + Cast<usize>,\n    {\n        // \u6587\u5B57\u5217 s \u306E\u672B\u5C3E\
    \u306B\u30C0\u30DF\u30FC\u6587\u5B57(\u6700\u5C0F\u306E\u6587\u5B57)\u3092\u8FFD\
    \u52A0\u3057\u3066\u3001\u6570\u5217 s \u3068\u3059\u308B\n        let n = s.len()\
    \ + 1;\n        let mut s = if n == 1 {\n            vec![0]\n        } else {\n\
    \            let min = *s.iter().min().unwrap();\n            let max = *s.iter().max().unwrap();\n\
    \            let m = (max - min).cast() + 1;\n            if m <= n {\n      \
    \          let mut encode = vec![0; m];\n                for &x in s {\n     \
    \               encode[(x - min).cast()] = 1;\n                }\n           \
    \     for i in 1..m {\n                    encode[i] += encode[i - 1];\n     \
    \           }\n                s.iter()\n                    .map(|&x| encode[(x\
    \ - min).cast()])\n                    .chain([0])\n                    .collect()\n\
    \            } else {\n                let mut z = s.to_vec();\n             \
    \   z.sort_unstable();\n                z.dedup();\n                s.iter()\n\
    \                    .map(|&x| z.binary_search(&x).unwrap() + 1)\n           \
    \         .chain([0])\n                    .collect()\n            }\n       \
    \ };\n\n        let mut sa = Self::sa_is(&s);\n        // \u30C0\u30DF\u30FC\u6587\
    \u5B57\u3092\u6D88\u3059\n        s.pop();\n        sa.remove(0);\n\n        let\
    \ sa_inv = Self::inverse_permutation(&sa);\n\n        let lcp = Self::build_lcp_array(&s,\
    \ &sa, &sa_inv);\n        let rmq = RangeMinimumQuery::from_iter(lcp.clone());\n\
    \        Self {\n            sa,\n            sa_inv,\n            lcp,\n    \
    \        rmq,\n        }\n    }\n\n    pub fn suffix_array(&self) -> &[usize]\
    \ {\n        &self.sa\n    }\n\n    pub fn suffix_array_inv(&self) -> &[usize]\
    \ {\n        &self.sa_inv\n    }\n\n    pub fn lcp_array(&self) -> &[usize] {\n\
    \        &self.lcp\n    }\n\n    pub fn lcp(&self, i: usize, j: usize) -> usize\
    \ {\n        let n = self.sa.len();\n        if i == n || j == n {\n         \
    \   return 0;\n        }\n        if i == j {\n            return n - i;\n   \
    \     }\n        let mut i = self.sa_inv[i];\n        let mut j = self.sa_inv[j];\n\
    \        if i > j {\n            std::mem::swap(&mut i, &mut j);\n        }\n\
    \        *self.rmq.min(i..j).unwrap()\n    }\n\n    pub fn compare(\n        &self,\n\
    \        s1_range: impl RangeBounds<usize>,\n        s2_range: impl RangeBounds<usize>,\n\
    \    ) -> Ordering {\n        let (l1, r1) = s1_range.into_half_open_range(0,\
    \ self.sa.len());\n        let (l2, r2) = s2_range.into_half_open_range(0, self.sa.len());\n\
    \        let n1 = r1 - l1;\n        let n2 = r2 - l2;\n        let lcp = self.lcp(l1,\
    \ l2);\n        match (n1 == lcp, n2 == lcp) {\n            (true, true) => Ordering::Equal,\n\
    \            (true, false) => Ordering::Less,\n            (false, true) => Ordering::Greater,\n\
    \            (false, false) => self.sa_inv[l1 + lcp].cmp(&self.sa_inv[l2 + lcp]),\n\
    \        }\n    }\n\n    fn sa_is(s: &[usize]) -> Vec<usize> {\n        let n\
    \ = s.len();\n\n        let mut count = vec![0; n];\n        for &x in s {\n \
    \           count[x] += 1;\n        }\n        if count.iter().all(|&x| x == 1)\
    \ {\n            return Self::inverse_permutation(s);\n        }\n\n        let\
    \ ls = Self::classify(s);\n\n        // s-type \u3092\u9069\u5F53\u306A\u9806\u3067\
    \ bucket \u306E\u672B\u5C3E\u306B\u5165\u308C\u308B\n        let mut sa = vec![!0;\
    \ n];\n        let mut tail = Self::bucket_tail(&count);\n        for i in (1..n).rev().filter(|&i|\
    \ ls[i] == Type::S(true)) {\n            tail[s[i]] -= 1;\n            sa[tail[s[i]]]\
    \ = i;\n        }\n\n        Self::induce(s, &mut sa, &count, &ls);\n\n      \
    \  let lms: Vec<_> = sa\n            .into_iter()\n            .filter(|&i| i\
    \ != !0 && ls[i] == Type::S(true))\n            .collect();\n        let rs_sa\
    \ = Self::sa_is(&Self::reduce(s, &lms, &ls));\n\n        let lms: Vec<_> = (0..n).filter(|&i|\
    \ ls[i] == Type::S(true)).collect();\n\n        let mut tail = Self::bucket_tail(&count);\n\
    \        let mut sa = vec![!0; n];\n        for i in rs_sa.into_iter().rev() {\n\
    \            let j = lms[i];\n            tail[s[j]] -= 1;\n            sa[tail[s[j]]]\
    \ = j;\n        }\n\n        Self::induce(s, &mut sa, &count, &ls);\n\n      \
    \  sa\n    }\n\n    fn inverse_permutation(s: &[usize]) -> Vec<usize> {\n    \
    \    let n = s.len();\n        let mut res = vec![0; n];\n        for i in 0..n\
    \ {\n            res[s[i]] = i;\n        }\n        res\n    }\n\n    fn classify(s:\
    \ &[usize]) -> Vec<Type> {\n        let n = s.len();\n        let mut ls = vec![Type::S(false);\
    \ n];\n        for i in (0..n - 1).rev() {\n            ls[i] = match s[i].cmp(&s[i\
    \ + 1]) {\n                Ordering::Less => Type::S(false),\n               \
    \ Ordering::Equal => ls[i + 1],\n                Ordering::Greater => Type::L,\n\
    \            };\n        }\n        for i in 1..n {\n            if let (Type::L,\
    \ Type::S(_)) = (ls[i - 1], ls[i]) {\n                ls[i] = Type::S(true); //\
    \ leftmost S-type\n            }\n        }\n        ls\n    }\n\n    fn bucket_head(count:\
    \ &[usize]) -> Vec<usize> {\n        std::iter::once(&0)\n            .chain(&count[..count.len()\
    \ - 1])\n            .scan(0, |acc, &x| {\n                *acc += x;\n      \
    \          Some(*acc)\n            })\n            .collect()\n    }\n\n    fn\
    \ bucket_tail(count: &[usize]) -> Vec<usize> {\n        count\n            .iter()\n\
    \            .scan(0, |acc, &x| {\n                *acc += x;\n              \
    \  Some(*acc)\n            })\n            .collect()\n    }\n\n    fn induce(s:\
    \ &[usize], sa: &mut [usize], count: &[usize], ls: &[Type]) {\n        let n =\
    \ s.len();\n\n        let mut head = Self::bucket_head(&count);\n        for i\
    \ in 0..n {\n            if sa[i] == 0 || sa[i] == !0 || ls[sa[i] - 1] != Type::L\
    \ {\n                continue;\n            }\n            let j = sa[i] - 1;\n\
    \            sa[head[s[j]]] = j;\n            head[s[j]] += 1;\n        }\n\n\
    \        let mut tail = Self::bucket_tail(&count);\n        for i in (1..n).rev()\
    \ {\n            if sa[i] == 0 || sa[i] == !0 || ls[sa[i] - 1] == Type::L {\n\
    \                continue;\n            }\n            let j = sa[i] - 1;\n  \
    \          tail[s[j]] -= 1;\n            sa[tail[s[j]]] = j;\n        }\n    }\n\
    \n    fn reduce(s: &[usize], lms: &[usize], ls: &[Type]) -> Vec<usize> {\n   \
    \     if lms.len() <= 1 {\n            return vec![0; lms.len()];\n        }\n\
    \n        let mut map = vec![0; s.len()];\n        map[lms[1]] = 1;\n        let\
    \ mut x = 1;\n        for i in 2..lms.len() {\n            let equiv = s[lms[i]]\
    \ == s[lms[i - 1]]\n                && (lms[i] + 1..)\n                    .zip(lms[i\
    \ - 1] + 1..)\n                    .find_map(|(i0, i1)| {\n                  \
    \      if (ls[i0], ls[i1]) == (Type::S(true), Type::S(true)) {\n             \
    \               Some(true)\n                        } else if ls[i0] != ls[i1]\
    \ || s[i0] != s[i1] {\n                            Some(false)\n             \
    \           } else {\n                            None\n                     \
    \   }\n                    })\n                    .unwrap();\n            if\
    \ !equiv {\n                x += 1;\n            }\n            map[lms[i]] =\
    \ x;\n        }\n\n        (0..s.len())\n            .filter_map(|i| (ls[i] ==\
    \ Type::S(true)).then_some(map[i]))\n            .collect()\n    }\n\n    fn build_lcp_array(s:\
    \ &[usize], sa: &[usize], sa_inv: &[usize]) -> Vec<usize> {\n        let n = s.len();\n\
    \        if n == 0 {\n            return vec![];\n        }\n\n        let mut\
    \ h = 0;\n        let mut lcp = vec![0; n - 1];\n        for i in 0..n - 1 {\n\
    \            if h > 0 {\n                h -= 1;\n            }\n            if\
    \ sa_inv[i] == 0 {\n                continue;\n            }\n            let\
    \ j = sa[sa_inv[i] - 1];\n            while i + h < n && j + h < n {\n       \
    \         if s[i + h] != s[j + h] {\n                    break;\n            \
    \    }\n                h += 1;\n            }\n            lcp[sa_inv[i] - 1]\
    \ = h;\n        }\n        lcp\n    }\n}\n\n#[derive(Clone, Copy, PartialEq, Eq)]\n\
    enum Type {\n    L,\n    S(bool), // true: leftmost S-type\n}\n"
  dependsOn:
  - crates/algebra/numeric_traits/src/cast.rs
  - crates/algebra/numeric_traits/src/inf.rs
  - crates/algebra/numeric_traits/src/integer.rs
  - crates/algebra/numeric_traits/src/lib.rs
  - crates/algebra/numeric_traits/src/numeric.rs
  - crates/algebra/numeric_traits/src/signed.rs
  - crates/algebra/numeric_traits/src/zero_one.rs
  - crates/data_structure/range_minimum_query/src/lib.rs
  - crates/misc/into_half_open_range/src/lib.rs
  isVerificationFile: false
  path: crates/string/suffix_array/src/lib.rs
  requiredBy: []
  timestamp: '2025-03-23 00:32:36+00:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/library_checker/string/suffixarray/src/main.rs
documentation_of: crates/string/suffix_array/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/suffix_array/src/lib.rs
- /library/crates/string/suffix_array/src/lib.rs.html
title: crates/string/suffix_array/src/lib.rs
---
