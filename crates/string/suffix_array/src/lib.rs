// https://rsk0315.github.io/library-rs/nekolib/seq/suffix_array/struct.SuffixArray.html#fnref2

use std::{cmp::Ordering, ops::RangeBounds};

use as_half_open_range::AsHalfOpenRange;
use numeric_traits::{Cast, Integer};
use range_minimum_query::RangeMinimumQuery;

#[derive(Clone)]
pub struct SuffixArray {
    sa: Vec<usize>,
    sa_inv: Vec<usize>,
    lcp: Vec<usize>,
    rmq: RangeMinimumQuery<usize>,
}

impl SuffixArray {
    pub fn new<T>(s: &[T]) -> Self
    where
        T: Integer + Cast<usize>,
    {
        // 文字列 s の末尾にダミー文字(最小の文字)を追加して、数列 s とする
        let n = s.len() + 1;
        let mut s = if n == 1 {
            vec![0]
        } else {
            let min = *s.iter().min().unwrap();
            let max = *s.iter().max().unwrap();
            let m = (max - min).cast() + 1;
            if m <= n {
                let mut encode = vec![0; m];
                for &x in s {
                    encode[(x - min).cast()] = 1;
                }
                for i in 1..m {
                    encode[i] += encode[i - 1];
                }
                s.iter()
                    .map(|&x| encode[(x - min).cast()])
                    .chain([0])
                    .collect()
            } else {
                let mut z = s.to_vec();
                z.sort_unstable();
                z.dedup();
                s.iter()
                    .map(|&x| z.binary_search(&x).unwrap() + 1)
                    .chain([0])
                    .collect()
            }
        };

        let mut sa = Self::sa_is(&s);
        // ダミー文字を消す
        s.pop();
        sa.remove(0);

        let sa_inv = Self::inverse_permutation(&sa);

        let lcp = Self::build_lcp_array(&s, &sa, &sa_inv);
        let rmq = RangeMinimumQuery::from_iter(lcp.clone());
        Self {
            sa,
            sa_inv,
            lcp,
            rmq,
        }
    }

    pub fn suffix_array(&self) -> &[usize] {
        &self.sa
    }

    pub fn suffix_array_inv(&self) -> &[usize] {
        &self.sa_inv
    }

    pub fn lcp_array(&self) -> &[usize] {
        &self.lcp
    }

    pub fn lcp(&self, i: usize, j: usize) -> usize {
        let n = self.sa.len();
        if i == n || j == n {
            return 0;
        }
        if i == j {
            return n - i;
        }
        let mut i = self.sa_inv[i];
        let mut j = self.sa_inv[j];
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        *self.rmq.min(i..j).unwrap()
    }

    pub fn compare(
        &self,
        s1_range: impl RangeBounds<usize>,
        s2_range: impl RangeBounds<usize>,
    ) -> Ordering {
        let (l1, r1) = s1_range.as_half_open_range(0, self.sa.len());
        let (l2, r2) = s2_range.as_half_open_range(0, self.sa.len());
        let n1 = r1 - l1;
        let n2 = r2 - l2;
        let lcp = self.lcp(l1, l2);
        match (n1 == lcp, n2 == lcp) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => self.sa_inv[l1 + lcp].cmp(&self.sa_inv[l2 + lcp]),
        }
    }

    fn sa_is(s: &[usize]) -> Vec<usize> {
        let n = s.len();

        let mut count = vec![0; n];
        for &x in s {
            count[x] += 1;
        }
        if count.iter().all(|&x| x == 1) {
            return Self::inverse_permutation(s);
        }

        let ls = Self::classify(s);

        // s-type を適当な順で bucket の末尾に入れる
        let mut sa = vec![!0; n];
        let mut tail = Self::bucket_tail(&count);
        for i in (1..n).rev().filter(|&i| ls[i] == Type::S(true)) {
            tail[s[i]] -= 1;
            sa[tail[s[i]]] = i;
        }

        Self::induce(s, &mut sa, &count, &ls);

        let lms: Vec<_> = sa
            .into_iter()
            .filter(|&i| i != !0 && ls[i] == Type::S(true))
            .collect();
        let rs_sa = Self::sa_is(&Self::reduce(s, &lms, &ls));

        let lms: Vec<_> = (0..n).filter(|&i| ls[i] == Type::S(true)).collect();

        let mut tail = Self::bucket_tail(&count);
        let mut sa = vec![!0; n];
        for i in rs_sa.into_iter().rev() {
            let j = lms[i];
            tail[s[j]] -= 1;
            sa[tail[s[j]]] = j;
        }

        Self::induce(s, &mut sa, &count, &ls);

        sa
    }

    fn inverse_permutation(s: &[usize]) -> Vec<usize> {
        let n = s.len();
        let mut res = vec![0; n];
        for i in 0..n {
            res[s[i]] = i;
        }
        res
    }

    fn classify(s: &[usize]) -> Vec<Type> {
        let n = s.len();
        let mut ls = vec![Type::S(false); n];
        for i in (0..n - 1).rev() {
            ls[i] = match s[i].cmp(&s[i + 1]) {
                Ordering::Less => Type::S(false),
                Ordering::Equal => ls[i + 1],
                Ordering::Greater => Type::L,
            };
        }
        for i in 1..n {
            if let (Type::L, Type::S(_)) = (ls[i - 1], ls[i]) {
                ls[i] = Type::S(true); // leftmost S-type
            }
        }
        ls
    }

    fn bucket_head(count: &[usize]) -> Vec<usize> {
        std::iter::once(&0)
            .chain(&count[..count.len() - 1])
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }

    fn bucket_tail(count: &[usize]) -> Vec<usize> {
        count
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect()
    }

    fn induce(s: &[usize], sa: &mut [usize], count: &[usize], ls: &[Type]) {
        let n = s.len();

        let mut head = Self::bucket_head(count);
        for i in 0..n {
            if sa[i] == 0 || sa[i] == !0 || ls[sa[i] - 1] != Type::L {
                continue;
            }
            let j = sa[i] - 1;
            sa[head[s[j]]] = j;
            head[s[j]] += 1;
        }

        let mut tail = Self::bucket_tail(count);
        for i in (1..n).rev() {
            if sa[i] == 0 || sa[i] == !0 || ls[sa[i] - 1] == Type::L {
                continue;
            }
            let j = sa[i] - 1;
            tail[s[j]] -= 1;
            sa[tail[s[j]]] = j;
        }
    }

    fn reduce(s: &[usize], lms: &[usize], ls: &[Type]) -> Vec<usize> {
        if lms.len() <= 1 {
            return vec![0; lms.len()];
        }

        let mut map = vec![0; s.len()];
        map[lms[1]] = 1;
        let mut x = 1;
        for i in 2..lms.len() {
            let equiv = s[lms[i]] == s[lms[i - 1]]
                && (lms[i] + 1..)
                    .zip(lms[i - 1] + 1..)
                    .find_map(|(i0, i1)| {
                        if (ls[i0], ls[i1]) == (Type::S(true), Type::S(true)) {
                            Some(true)
                        } else if ls[i0] != ls[i1] || s[i0] != s[i1] {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .unwrap();
            if !equiv {
                x += 1;
            }
            map[lms[i]] = x;
        }

        (0..s.len())
            .filter_map(|i| (ls[i] == Type::S(true)).then_some(map[i]))
            .collect()
    }

    fn build_lcp_array(s: &[usize], sa: &[usize], sa_inv: &[usize]) -> Vec<usize> {
        let n = s.len();
        if n == 0 {
            return vec![];
        }

        let mut h = 0usize;
        let mut lcp = vec![0; n - 1];
        for i in 0..n - 1 {
            h = h.saturating_sub(1);
            if sa_inv[i] == 0 {
                continue;
            }
            let j = sa[sa_inv[i] - 1];
            while i + h < n && j + h < n {
                if s[i + h] != s[j + h] {
                    break;
                }
                h += 1;
            }
            lcp[sa_inv[i] - 1] = h;
        }
        lcp
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Type {
    L,
    S(bool), // true: leftmost S-type
}
