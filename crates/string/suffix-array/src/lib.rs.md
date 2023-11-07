---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/algebraic/algebraic/src/lib.rs
    title: crates/algebraic/algebraic/src/lib.rs
  - icon: ':heavy_check_mark:'
    path: crates/data-structure/disjoint-sparse-table/src/lib.rs
    title: crates/data-structure/disjoint-sparse-table/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: verify/number_of_substrings/src/main.rs
    title: verify/number_of_substrings/src/main.rs
  - icon: ':heavy_check_mark:'
    path: verify/suffixarray/src/main.rs
    title: verify/suffixarray/src/main.rs
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
  code: "use std::{\n    cell::RefCell,\n    mem::swap,\n    ops::{Add, Sub},\n};\n\
    \nuse algebraic::{algebra, monoid};\nuse disjoint_sparse_table::DisjointSparseTable;\n\
    \nconst THRESHOLD_NAIVE: usize = 10;\nconst THRESHOLD_DOUBLING: usize = 40;\n\
    const THRESHOLD_COMPRESS: usize = 16777216;\n\n#[allow(dead_code)]\npub struct\
    \ SuffixArray<'a, T>\nwhere\n    T: Copy + Ord + Add<T> + Sub<T> + Into<usize>,\n\
    {\n    s: &'a [T],\n    n: usize,\n    sa: Vec<usize>,\n    rank: Vec<usize>,\n\
    \    lcp: Vec<usize>,\n    spt: RefCell<Option<DisjointSparseTable<M>>>,\n}\n\n\
    impl<'a, T> SuffixArray<'a, T>\nwhere\n    T: Copy + Ord + Add<T, Output = T>\
    \ + Sub<T, Output = T> + Into<usize>,\n{\n    pub fn build(s: &'a [T]) -> Self\
    \ {\n        let n = s.len();\n        if n == 0 {\n            return Self {\n\
    \                s,\n                n,\n                sa: vec![],\n       \
    \         rank: vec![],\n                lcp: vec![],\n                spt: RefCell::new(None),\n\
    \            };\n        }\n        let min = *s.iter().min().unwrap();\n    \
    \    let max = *s.iter().max().unwrap();\n        let m = (max - min).into();\n\
    \        let t: Vec<_> = if m < THRESHOLD_COMPRESS {\n            s.iter().map(|&x|\
    \ (x - min).into()).collect()\n        } else {\n            let mut idx = (0..n).collect::<Vec<_>>();\n\
    \            idx.sort_by_key(|&i| s[i]);\n            let mut c = 0;\n       \
    \     let mut t = vec![0; n];\n            for i in 0..n {\n                if\
    \ i > 0 && s[idx[i - 1]] != s[idx[i]] {\n                    c += 1;\n       \
    \         }\n                t[idx[i]] = c;\n            }\n            t\n  \
    \      };\n        let sa = sa_is(&t, m);\n        let mut rank = vec![0; n];\n\
    \        for i in 0..n {\n            rank[sa[i]] = i;\n        }\n        let\
    \ lcp = lcp_array(&t, &sa, &rank);\n        Self {\n            s,\n         \
    \   n,\n            sa,\n            rank,\n            lcp,\n            spt:\
    \ RefCell::new(None),\n        }\n    }\n\n    pub fn suffix_array(&self) -> &[usize]\
    \ {\n        &self.sa\n    }\n\n    pub fn rank(&self) -> &[usize] {\n       \
    \ &self.rank\n    }\n\n    pub fn lcp_array(&self) -> &[usize] {\n        &self.lcp\n\
    \    }\n\n    pub fn lcp(&self, i: usize, j: usize) -> usize {\n        assert!(i\
    \ <= self.n && j <= self.n);\n        if i == self.n || j == self.n {\n      \
    \      0\n        } else if i == j {\n            self.n - i\n        } else {\n\
    \            let mut i = self.rank[i];\n            let mut j = self.rank[j];\n\
    \            if i > j {\n                swap(&mut i, &mut j);\n            }\n\
    \            if self.spt.borrow().is_none() {\n                self.spt\n    \
    \                .replace(Some(DisjointSparseTable::<M>::new(&self.lcp)));\n \
    \           }\n            self.spt.borrow().as_ref().unwrap().prod(i, j)\n  \
    \      }\n    }\n}\n\nalgebra!(M, usize);\nmonoid!(M, !0, |x: &usize, y: &usize|\
    \ *x.min(y));\n\nfn sa_naive(s: &[usize]) -> Vec<usize> {\n    let n = s.len();\n\
    \    let mut sa: Vec<usize> = (0..n).collect();\n    sa.sort_by(|&(mut l), &(mut\
    \ r)| {\n        if l == r {\n            return std::cmp::Ordering::Equal;\n\
    \        }\n        while l < n && r < n {\n            if s[l] != s[r] {\n  \
    \              return s[l].cmp(&s[r]);\n            }\n            l += 1;\n \
    \           r += 1;\n        }\n        if l == n {\n            std::cmp::Ordering::Less\n\
    \        } else {\n            std::cmp::Ordering::Greater\n        }\n    });\n\
    \    sa\n}\n\nfn sa_doubling(s: &[usize]) -> Vec<usize> {\n    let n = s.len();\n\
    \    let mut sa: Vec<_> = (0..n).collect();\n    let mut rnk: Vec<_> = s.iter().map(|&x|\
    \ x + 1).collect();\n    let mut tmp = vec![0; n];\n    let mut k = 1;\n    while\
    \ k < n {\n        let cmp = |&x: &usize, &y: &usize| {\n            if rnk[x]\
    \ != rnk[y] {\n                return rnk[x].cmp(&rnk[y]);\n            }\n  \
    \          let rx = if x + k < n { rnk[x + k] } else { 0 };\n            let ry\
    \ = if y + k < n { rnk[y + k] } else { 0 };\n            rx.cmp(&ry)\n       \
    \ };\n        sa.sort_by(cmp);\n        tmp[sa[0]] = 0;\n        for i in 1..n\
    \ {\n            tmp[sa[i]] =\n                tmp[sa[i - 1]] + usize::from(cmp(&sa[i\
    \ - 1], &sa[i]) == std::cmp::Ordering::Less);\n        }\n        std::mem::swap(&mut\
    \ tmp, &mut rnk);\n        k *= 2;\n    }\n    sa\n}\n\nfn sa_is(s: &[usize],\
    \ upper: usize) -> Vec<usize> {\n    let n = s.len();\n    match n {\n       \
    \ 0 => return vec![],\n        1 => return vec![0],\n        2 => return if s[0]\
    \ < s[1] { vec![0, 1] } else { vec![1, 0] },\n        n if n < THRESHOLD_NAIVE\
    \ => return sa_naive(s),\n        n if n < THRESHOLD_DOUBLING => return sa_doubling(s),\n\
    \        _ => (),\n    }\n    let mut sa = vec![0; n];\n    let mut ls = vec![false;\
    \ n];\n    for i in (0..n - 1).rev() {\n        ls[i] = if s[i] == s[i + 1] {\n\
    \            ls[i + 1]\n        } else {\n            s[i] < s[i + 1]\n      \
    \  };\n    }\n    let mut sum_l = vec![0; upper + 1];\n    let mut sum_s = vec![0;\
    \ upper + 1];\n    for i in 0..n {\n        if !ls[i] {\n            sum_s[s[i]]\
    \ += 1;\n        } else {\n            sum_l[s[i] + 1] += 1;\n        }\n    }\n\
    \    for i in 0..=upper {\n        sum_s[i] += sum_l[i];\n        if i < upper\
    \ {\n            sum_l[i + 1] += sum_s[i];\n        }\n    }\n    let induce =\
    \ |sa: &mut [usize], lms: &[usize]| {\n        for elem in sa.iter_mut() {\n \
    \           *elem = 0;\n        }\n        let mut buf = sum_s.clone();\n    \
    \    for &d in lms {\n            if d == n {\n                continue;\n   \
    \         }\n            let old = buf[s[d]];\n            buf[s[d]] += 1;\n \
    \           sa[old] = d + 1;\n        }\n        buf.copy_from_slice(&sum_l);\n\
    \        let old = buf[s[n - 1]];\n        buf[s[n - 1]] += 1;\n        sa[old]\
    \ = n;\n        for i in 0..n {\n            let v = sa[i];\n            if v\
    \ >= 2 && !ls[v - 2] {\n                let old = buf[s[v - 2]];\n           \
    \     buf[s[v - 2]] += 1;\n                sa[old] = v - 1;\n            }\n \
    \       }\n        buf.copy_from_slice(&sum_l);\n        for i in (0..n).rev()\
    \ {\n            let v = sa[i];\n            if v >= 2 && ls[v - 2] {\n      \
    \          buf[s[v - 2] + 1] -= 1;\n                sa[buf[s[v - 2] + 1]] = v\
    \ - 1;\n            }\n        }\n    };\n    let mut lms_map = vec![0; n + 1];\n\
    \    let mut m = 0;\n    for i in 1..n {\n        if !ls[i - 1] && ls[i] {\n \
    \           lms_map[i] = m + 1;\n            m += 1;\n        }\n    }\n    let\
    \ mut lms = Vec::with_capacity(m);\n    for i in 1..n {\n        if !ls[i - 1]\
    \ && ls[i] {\n            lms.push(i);\n        }\n    }\n    assert_eq!(lms.len(),\
    \ m);\n    induce(&mut sa, &lms);\n    if m > 0 {\n        let mut sorted_lms\
    \ = Vec::with_capacity(m);\n        for &v in &sa {\n            if lms_map[v\
    \ - 1] != 0 {\n                sorted_lms.push(v - 1);\n            }\n      \
    \  }\n        let mut rec_s = vec![0; m];\n        let mut rec_upper = 0;\n  \
    \      rec_s[lms_map[sorted_lms[0]] - 1] = 0;\n        for i in 1..m {\n     \
    \       let mut l = sorted_lms[i - 1];\n            let mut r = sorted_lms[i];\n\
    \            let end_l = if lms_map[l] < m { lms[lms_map[l]] } else { n };\n \
    \           let end_r = if lms_map[r] < m { lms[lms_map[r]] } else { n };\n  \
    \          let same = if end_l - l != end_r - r {\n                false\n   \
    \         } else {\n                while l < end_l {\n                    if\
    \ s[l] != s[r] {\n                        break;\n                    }\n    \
    \                l += 1;\n                    r += 1;\n                }\n   \
    \             l != n && s[l] == s[r]\n            };\n            if !same {\n\
    \                rec_upper += 1;\n            }\n            rec_s[lms_map[sorted_lms[i]]\
    \ - 1] = rec_upper;\n        }\n        let rec_sa = sa_is(&rec_s, rec_upper);\n\
    \        for i in 0..m {\n            sorted_lms[i] = lms[rec_sa[i]];\n      \
    \  }\n        induce(&mut sa, &mut sorted_lms);\n    }\n    for elem in sa.iter_mut()\
    \ {\n        *elem -= 1;\n    }\n    sa\n}\n\nfn lcp_array(s: &[usize], sa: &[usize],\
    \ rank: &[usize]) -> Vec<usize> {\n    let n = s.len();\n    assert!(n >= 1);\n\
    \    let mut lcp = vec![0; n - 1];\n    let mut h: usize = 0;\n    for i in 0..n\
    \ - 1 {\n        h = h.saturating_sub(1);\n        if rank[i] == 0 {\n       \
    \     continue;\n        }\n        let j = sa[rank[i] - 1];\n        while j\
    \ + h < n && i + h < n {\n            if s[j + h] != s[i + h] {\n            \
    \    break;\n            }\n            h += 1;\n        }\n        lcp[rank[i]\
    \ - 1] = h;\n    }\n    lcp\n}\n"
  dependsOn:
  - crates/algebraic/algebraic/src/lib.rs
  - crates/data-structure/disjoint-sparse-table/src/lib.rs
  isVerificationFile: false
  path: crates/string/suffix-array/src/lib.rs
  requiredBy: []
  timestamp: '2023-06-13 17:07:21+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - verify/number_of_substrings/src/main.rs
  - verify/suffixarray/src/main.rs
documentation_of: crates/string/suffix-array/src/lib.rs
layout: document
redirect_from:
- /library/crates/string/suffix-array/src/lib.rs
- /library/crates/string/suffix-array/src/lib.rs.html
title: crates/string/suffix-array/src/lib.rs
---
