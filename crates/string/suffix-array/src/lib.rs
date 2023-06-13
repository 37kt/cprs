use std::{
    cell::RefCell,
    mem::swap,
    ops::{Add, Sub},
};

use algebraic::{algebra, monoid};
use disjoint_sparse_table::DisjointSparseTable;

const THRESHOLD_NAIVE: usize = 10;
const THRESHOLD_DOUBLING: usize = 40;
const THRESHOLD_COMPRESS: usize = 16777216;

#[allow(dead_code)]
pub struct SuffixArray<'a, T>
where
    T: Copy + Ord + Add<T> + Sub<T> + Into<usize>,
{
    s: &'a [T],
    n: usize,
    sa: Vec<usize>,
    rank: Vec<usize>,
    lcp: Vec<usize>,
    spt: RefCell<Option<DisjointSparseTable<M>>>,
}

impl<'a, T> SuffixArray<'a, T>
where
    T: Copy + Ord + Add<T, Output = T> + Sub<T, Output = T> + Into<usize>,
{
    pub fn build(s: &'a [T]) -> Self {
        let n = s.len();
        if n == 0 {
            return Self {
                s,
                n,
                sa: vec![],
                rank: vec![],
                lcp: vec![],
                spt: RefCell::new(None),
            };
        }
        let min = *s.iter().min().unwrap();
        let max = *s.iter().max().unwrap();
        let m = (max - min).into();
        let t: Vec<_> = if m < THRESHOLD_COMPRESS {
            s.iter().map(|&x| (x - min).into()).collect()
        } else {
            let mut idx = (0..n).collect::<Vec<_>>();
            idx.sort_by_key(|&i| s[i]);
            let mut c = 0;
            let mut t = vec![0; n];
            for i in 0..n {
                if i > 0 && s[idx[i - 1]] != s[idx[i]] {
                    c += 1;
                }
                t[idx[i]] = c;
            }
            t
        };
        let sa = sa_is(&t, m);
        let mut rank = vec![0; n];
        for i in 0..n {
            rank[sa[i]] = i;
        }
        let lcp = lcp_array(&t, &sa, &rank);
        Self {
            s,
            n,
            sa,
            rank,
            lcp,
            spt: RefCell::new(None),
        }
    }

    pub fn suffix_array(&self) -> &[usize] {
        &self.sa
    }

    pub fn rank(&self) -> &[usize] {
        &self.rank
    }

    pub fn lcp_array(&self) -> &[usize] {
        &self.lcp
    }

    pub fn lcp(&self, i: usize, j: usize) -> usize {
        assert!(i <= self.n && j <= self.n);
        if i == self.n || j == self.n {
            0
        } else if i == j {
            self.n - i
        } else {
            let mut i = self.rank[i];
            let mut j = self.rank[j];
            if i > j {
                swap(&mut i, &mut j);
            }
            if self.spt.borrow().is_none() {
                self.spt
                    .replace(Some(DisjointSparseTable::<M>::new(&self.lcp)));
            }
            self.spt.borrow().as_ref().unwrap().prod(i, j)
        }
    }
}

algebra!(M, usize);
monoid!(M, !0, |x: &usize, y: &usize| *x.min(y));

fn sa_naive(s: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by(|&(mut l), &(mut r)| {
        if l == r {
            return std::cmp::Ordering::Equal;
        }
        while l < n && r < n {
            if s[l] != s[r] {
                return s[l].cmp(&s[r]);
            }
            l += 1;
            r += 1;
        }
        if l == n {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sa
}

fn sa_doubling(s: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<_> = (0..n).collect();
    let mut rnk: Vec<_> = s.iter().map(|&x| x + 1).collect();
    let mut tmp = vec![0; n];
    let mut k = 1;
    while k < n {
        let cmp = |&x: &usize, &y: &usize| {
            if rnk[x] != rnk[y] {
                return rnk[x].cmp(&rnk[y]);
            }
            let rx = if x + k < n { rnk[x + k] } else { 0 };
            let ry = if y + k < n { rnk[y + k] } else { 0 };
            rx.cmp(&ry)
        };
        sa.sort_by(cmp);
        tmp[sa[0]] = 0;
        for i in 1..n {
            tmp[sa[i]] =
                tmp[sa[i - 1]] + usize::from(cmp(&sa[i - 1], &sa[i]) == std::cmp::Ordering::Less);
        }
        std::mem::swap(&mut tmp, &mut rnk);
        k *= 2;
    }
    sa
}

fn sa_is(s: &[usize], upper: usize) -> Vec<usize> {
    let n = s.len();
    match n {
        0 => return vec![],
        1 => return vec![0],
        2 => return if s[0] < s[1] { vec![0, 1] } else { vec![1, 0] },
        n if n < THRESHOLD_NAIVE => return sa_naive(s),
        n if n < THRESHOLD_DOUBLING => return sa_doubling(s),
        _ => (),
    }
    let mut sa = vec![0; n];
    let mut ls = vec![false; n];
    for i in (0..n - 1).rev() {
        ls[i] = if s[i] == s[i + 1] {
            ls[i + 1]
        } else {
            s[i] < s[i + 1]
        };
    }
    let mut sum_l = vec![0; upper + 1];
    let mut sum_s = vec![0; upper + 1];
    for i in 0..n {
        if !ls[i] {
            sum_s[s[i]] += 1;
        } else {
            sum_l[s[i] + 1] += 1;
        }
    }
    for i in 0..=upper {
        sum_s[i] += sum_l[i];
        if i < upper {
            sum_l[i + 1] += sum_s[i];
        }
    }
    let induce = |sa: &mut [usize], lms: &[usize]| {
        for elem in sa.iter_mut() {
            *elem = 0;
        }
        let mut buf = sum_s.clone();
        for &d in lms {
            if d == n {
                continue;
            }
            let old = buf[s[d]];
            buf[s[d]] += 1;
            sa[old] = d + 1;
        }
        buf.copy_from_slice(&sum_l);
        let old = buf[s[n - 1]];
        buf[s[n - 1]] += 1;
        sa[old] = n;
        for i in 0..n {
            let v = sa[i];
            if v >= 2 && !ls[v - 2] {
                let old = buf[s[v - 2]];
                buf[s[v - 2]] += 1;
                sa[old] = v - 1;
            }
        }
        buf.copy_from_slice(&sum_l);
        for i in (0..n).rev() {
            let v = sa[i];
            if v >= 2 && ls[v - 2] {
                buf[s[v - 2] + 1] -= 1;
                sa[buf[s[v - 2] + 1]] = v - 1;
            }
        }
    };
    let mut lms_map = vec![0; n + 1];
    let mut m = 0;
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            lms_map[i] = m + 1;
            m += 1;
        }
    }
    let mut lms = Vec::with_capacity(m);
    for i in 1..n {
        if !ls[i - 1] && ls[i] {
            lms.push(i);
        }
    }
    assert_eq!(lms.len(), m);
    induce(&mut sa, &lms);
    if m > 0 {
        let mut sorted_lms = Vec::with_capacity(m);
        for &v in &sa {
            if lms_map[v - 1] != 0 {
                sorted_lms.push(v - 1);
            }
        }
        let mut rec_s = vec![0; m];
        let mut rec_upper = 0;
        rec_s[lms_map[sorted_lms[0]] - 1] = 0;
        for i in 1..m {
            let mut l = sorted_lms[i - 1];
            let mut r = sorted_lms[i];
            let end_l = if lms_map[l] < m { lms[lms_map[l]] } else { n };
            let end_r = if lms_map[r] < m { lms[lms_map[r]] } else { n };
            let same = if end_l - l != end_r - r {
                false
            } else {
                while l < end_l {
                    if s[l] != s[r] {
                        break;
                    }
                    l += 1;
                    r += 1;
                }
                l != n && s[l] == s[r]
            };
            if !same {
                rec_upper += 1;
            }
            rec_s[lms_map[sorted_lms[i]] - 1] = rec_upper;
        }
        let rec_sa = sa_is(&rec_s, rec_upper);
        for i in 0..m {
            sorted_lms[i] = lms[rec_sa[i]];
        }
        induce(&mut sa, &mut sorted_lms);
    }
    for elem in sa.iter_mut() {
        *elem -= 1;
    }
    sa
}

fn lcp_array(s: &[usize], sa: &[usize], rank: &[usize]) -> Vec<usize> {
    let n = s.len();
    assert!(n >= 1);
    let mut lcp = vec![0; n - 1];
    let mut h: usize = 0;
    for i in 0..n - 1 {
        h = h.saturating_sub(1);
        if rank[i] == 0 {
            continue;
        }
        let j = sa[rank[i] - 1];
        while j + h < n && i + h < n {
            if s[j + h] != s[i + h] {
                break;
            }
            h += 1;
        }
        lcp[rank[i] - 1] = h;
    }
    lcp
}
