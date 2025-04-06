use std::ops::RangeBounds;

use as_half_open_range::AsHalfOpenRange;
use modint_61::ModInt61;

use crate::{random, RollingHash};

#[derive(Clone)]
pub struct RollingHashSequence {
    hash: Vec<ModInt61>,
    base_pow: Vec<ModInt61>,
}

impl<T> FromIterator<T> for RollingHashSequence
where
    T: Into<ModInt61>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut hash: Vec<_> = iter
            .into_iter()
            .map(|x| x.into() + random::<0x_ADD>())
            .chain([0.into()])
            .collect();
        let n = hash.len() - 1;
        for i in (0..n).rev() {
            hash[i] = hash[i] + hash[i + 1] * RollingHash::base();
        }

        let mut base_pow = Vec::with_capacity(n + 1);
        base_pow.push(ModInt61::from_raw(1));
        for i in 0..n {
            base_pow.push(base_pow[i] * RollingHash::base());
        }

        Self { hash, base_pow }
    }
}

impl RollingHashSequence {
    pub fn len(&self) -> usize {
        self.hash.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn base_pow(&self, exp: usize) -> ModInt61 {
        self.base_pow[exp]
    }

    pub fn range(&self, range: impl RangeBounds<usize>) -> RollingHash {
        let (l, r) = range.as_half_open_range(0, self.len());
        RollingHash {
            hash: self.hash[l] - self.hash[r] * self.base_pow[r - l],
            base_pow: self.base_pow[r - l],
        }
    }

    pub fn lcp(
        &self,
        range: impl RangeBounds<usize>,
        other: &RollingHashSequence,
        other_range: impl RangeBounds<usize>,
    ) -> usize {
        let (l1, r1) = range.as_half_open_range(0, self.len());
        let (l2, r2) = other_range.as_half_open_range(0, other.len());
        let max = (r1 - l1).min(r2 - l2);
        let mut ok = 0;
        let mut ng = max + 1;
        while ok + 1 < ng {
            let md = (ok + ng) / 2;
            if self.range(l1..l1 + md) == other.range(l2..l2 + md) {
                ok = md;
            } else {
                ng = md;
            }
        }
        ok
    }
}
