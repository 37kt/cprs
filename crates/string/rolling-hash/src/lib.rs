use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{Add, Bound, Mul, Neg, RangeBounds},
};

use modint61::ModInt61;
use nimber::Nimber;
use random::Pcg64Fast;

pub type RollingHashModInt61<'a, C> = RollingHash<'a, C, ModInt61>;
pub type RollingHashNimber<'a, C> = RollingHash<'a, C, Nimber>;

pub struct GenBaseImpl<T>(PhantomData<fn() -> T>);

pub trait GenBase {
    type H;
    fn base() -> Self::H;
}

impl GenBase for GenBaseImpl<Nimber> {
    type H = Nimber;

    fn base() -> Nimber {
        fn gen() -> Nimber {
            let mut rng = Pcg64Fast::default();
            Nimber::new(rng.u64())
        }

        thread_local! {
            static BASE: Nimber = gen();
        }
        BASE.with(|base| *base)
    }
}

impl GenBase for GenBaseImpl<ModInt61> {
    type H = ModInt61;

    fn base() -> ModInt61 {
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

        thread_local! {
            static BASE: ModInt61 = gen();
        }
        BASE.with(|base| *base)
    }
}

#[derive(Clone)]
pub struct RollingHash<'a, C, H>
where
    C: Copy + Eq + Into<H>,
    H: Copy + Eq + Add<Output = H> + Mul<Output = H> + Neg<Output = H>,
    GenBaseImpl<H>: GenBase<H = H>,
{
    s: &'a [C],
    hs: Box<[H]>,
    pw: Box<[H]>,
}

impl<'a, C, H> RollingHash<'a, C, H>
where
    C: Copy + Eq + Into<H>,
    H: Copy + Eq + From<u64> + Add<Output = H> + Mul<Output = H> + Neg<Output = H>,
    GenBaseImpl<H>: GenBase<H = H>,
{
    pub fn new(s: &'a [C]) -> Self {
        let n = s.len();
        let base = GenBaseImpl::<H>::base();
        let mut hs = vec![H::from(0); n + 1];
        let mut pw = vec![H::from(1); n + 1];
        for i in 0..n {
            hs[i + 1] = hs[i] * base + s[i].into();
            pw[i + 1] = pw[i] * base;
        }
        Self {
            s,
            hs: hs.into_boxed_slice(),
            pw: pw.into_boxed_slice(),
        }
    }

    pub fn base() -> H {
        GenBaseImpl::<H>::base()
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn get(&self, index: impl RangeBounds<usize>) -> H {
        let (l, r) = range_to_pair(index, self.len());
        self.hs[l] * -self.pw[r - l] + self.hs[r]
    }

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
}

impl<'a, C, H> RollingHash<'a, C, H>
where
    C: Copy + Eq + Into<H> + Ord,
    H: Copy + Eq + From<u64> + Add<Output = H> + Mul<Output = H> + Neg<Output = H>,
    GenBaseImpl<H>: GenBase<H = H>,
{
    pub fn compare(
        &self,
        index1: impl RangeBounds<usize>,
        other: &Self,
        index2: impl RangeBounds<usize>,
    ) -> Ordering {
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
