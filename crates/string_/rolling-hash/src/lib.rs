fn gen() -> ModInt61 {
    const FACTORS: [usize; 12] = [2, 3, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321];
    let mut rng = Pcg64Fast::default();
    loop {
        let x = ModInt61::new(rng.u64());
        if FACTORS
            .iter()
            .all(|&f| x.pow((ModInt61::modulus() as usize - 1) / f).val() > 1)
        {
            return x;
        }
    }
}

fn base() -> ModInt61 {
    thread_local! {
        static BASE: ModInt61 = gen();
    }
    BASE.with(|base| *base)
}

/// ローリングハッシュ
pub struct RollingHash {
    pow: RefCell<Vec<ModInt61>>,
}

impl RollingHash {
    pub fn new() -> Self {
        Self {
            pow: RefCell::new(vec![ModInt61::one(), base()]),
        }
    }

    pub fn base() -> ModInt61 {
        base()
    }

    /// 配列 `s` の部分列のハッシュを求めるためのテーブルを構築
    pub fn build_table<'a, 'b, T>(&'a self, s: &'b [T]) -> RollingHashTable<'a, 'b, T>
    where
        T: Clone + Into<ModInt61>,
    {
        let n = s.len();
        let mut hash = vec![ModInt61::zero(); n + 1];
        for i in 0..n {
            hash[i + 1] = hash[i] * self.pow(1) + s[i].clone().into();
        }
        RollingHashTable { rh: self, s, hash }
    }

    /// 配列 `s` のハッシュを求める
    pub fn hash<T>(s: &[T]) -> RollingHashedSequence
    where
        T: Clone + Into<ModInt61>,
    {
        RollingHashedSequence::from_slice(s)
    }

    fn expand(&self, n: usize) {
        let mut pow = self.pow.borrow_mut();
        for i in pow.len()..=n {
            let x = pow[i - 1] * base();
            pow.push(x);
        }
    }

    fn pow(&self, n: usize) -> ModInt61 {
        self.expand(n);
        self.pow.borrow()[n]
    }
}

#[derive(Clone)]
pub struct RollingHashTable<'a, 'b, T> {
    rh: &'a RollingHash,
    s: &'b [T],
    hash: Vec<ModInt61>,
}

impl<'a, 'b, T> RollingHashTable<'a, 'b, T> {
    /// 配列の長さを返す
    pub fn len(&self) -> usize {
        self.s.len()
    }

    /// 配列の部分列のハッシュを求める
    pub fn get(&self, index: impl RangeBounds<usize>) -> RollingHashedSequence {
        let (l, r) = range_to_pair(index, self.s.len());
        RollingHashedSequence {
            hash: self.hash[l] * -self.rh.pow(r - l) + self.hash[r],
            len: r - l,
            pow: self.rh.pow(r - l),
        }
    }

    /// 2 つの配列の最長共通接頭辞の長さを求める
    pub fn lcp(
        &self,
        index1: impl RangeBounds<usize>,
        other: &Self,
        index2: impl RangeBounds<usize>,
    ) -> usize {
        let (l1, r1) = range_to_pair(index1, self.len());
        let (l2, r2) = range_to_pair(index2, other.len());
        let n = (r1 - l1).min(r2 - l2);
        let mut ok = 0;
        let mut ng = n + 1;
        while ok + 1 < ng {
            let md = (ok + ng) / 2;
            if self.get(l1..l1 + md) == other.get(l2..l2 + md) {
                ok = md;
            } else {
                ng = md;
            }
        }
        ok
    }

    /// 2 つの配列の辞書順比較
    pub fn compare(
        &self,
        index1: impl RangeBounds<usize>,
        other: &Self,
        index2: impl RangeBounds<usize>,
    ) -> Ordering
    where
        T: Ord,
    {
        let (l1, r1) = range_to_pair(index1, self.len());
        let (l2, r2) = range_to_pair(index2, other.len());
        let n = self.lcp(l1..r1, other, l2..r2);
        if l1 + n == r1 {
            if l2 + n == r2 {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else if l2 + n == r2 {
            Ordering::Greater
        } else {
            self.s[l1 + n].cmp(&other.s[l2 + n])
        }
    }
}

/// ハッシュ化された数列
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RollingHashedSequence {
    hash: ModInt61,
    len: usize,
    pow: ModInt61,
}

impl Default for RollingHashedSequence {
    fn default() -> Self {
        Self {
            hash: ModInt61::zero(),
            len: 0,
            pow: ModInt61::one(),
        }
    }
}

impl RollingHashedSequence {
    /// 配列 `s` のハッシュを求める
    pub fn from_slice<T>(s: &[T]) -> Self
    where
        T: Clone + Into<ModInt61>,
    {
        let mut hash = ModInt61::zero();
        for c in s {
            hash = hash * base() + c.clone().into();
        }
        Self {
            hash,
            len: s.len(),
            pow: base().pow(s.len()),
        }
    }

    /// ハッシュを返す
    pub fn hash(self) -> ModInt61 {
        self.hash
    }

    /// 配列の長さを返す
    pub fn len(self) -> usize {
        self.len
    }

    /// 2 つのハッシュ化された数列を連結
    pub fn concat(self, other: Self) -> Self {
        Self {
            hash: self.hash * other.pow + other.hash,
            len: self.len.wrapping_add(other.len),
            pow: self.pow * other.pow,
        }
    }

    /// ハッシュ化された数列を `n` 回繰り返す
    pub fn repeat(self, n: usize) -> Self {
        RollingHashMonoid::pow(&self, n)
    }
}

#[derive(Clone, Copy)]
pub enum RollingHashMonoid {}

impl Algebra for RollingHashMonoid {
    type S = RollingHashedSequence;
}

impl Monoid for RollingHashMonoid {
    fn e() -> Self::S {
        Self::S::default()
    }

    fn op(a: &Self::S, b: &Self::S) -> Self::S {
        a.concat(*b)
    }
}

use std::{
    cell::RefCell,
    cmp::Ordering,
    ops::{Bound, RangeBounds},
};

use algebraic::{Algebra, Monoid, One, Zero};
use modint61::ModInt61;
use random::Pcg64Fast;

fn range_to_pair(range: impl RangeBounds<usize>, n: usize) -> (usize, usize) {
    let l = match range.start_bound() {
        Bound::Included(&l) => l,
        Bound::Excluded(&l) => l + 1,
        Bound::Unbounded => 0,
    };
    let r = match range.end_bound() {
        Bound::Included(&r) => r + 1,
        Bound::Excluded(&r) => r,
        Bound::Unbounded => n,
    };
    (l, r)
}
