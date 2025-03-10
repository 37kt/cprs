use std::ops::Range;

use numeric_traits::{Cast, Inf, Integer, NegInf};

use crate::WaveletMatrix2D;

impl<X, Y, const X_SMALL: bool, const Y_SMALL: bool, const INVERTIVE: bool>
    WaveletMatrix2D<X, Y, X_SMALL, Y_SMALL, INVERTIVE>
where
    X: Integer + Inf + NegInf + Cast<usize>,
    Y: Integer + Inf + NegInf + Cast<usize>,
    u32: Cast<X>,
    u32: Cast<Y>,
{
    pub(crate) fn count_with_(
        &self,
        xl: usize,
        xr: usize,
        yl: usize,
        yr: usize,
        mut f: impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        if INVERTIVE {
            self.count_prefix_with_(xl, xr, yr, &mut f)
                - self.count_prefix_with_(xl, xr, yl, |d, x, inv| f(d, x, !inv))
        } else {
            self.dfs_count_with_(self.lg, xl, xr, yl, yr, 0, 1 << self.lg, &mut f)
        }
    }

    fn dfs_count_with_(
        &self,
        d: usize,
        xl: usize,
        xr: usize,
        yl: usize,
        yr: usize,
        a: usize,
        b: usize,
        f: &mut impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        if yr <= a || b <= yl {
            return 0;
        } else if yl <= a && b <= yr {
            f(d, xl..xr, false);
            return xr - xl;
        }
        let d = d - 1;
        let c = (a + b) >> 1;
        let l0 = self.bv[d].count_prefix(xl, false);
        let r0 = self.bv[d].count_prefix(xr, false);
        let l1 = xl + self.mid[d] - l0;
        let r1 = xr + self.mid[d] - r0;
        self.dfs_count_with_(d, l0, r0, yl, yr, a, c, f)
            + self.dfs_count_with_(d, l1, r1, yl, yr, c, b, f)
    }

    fn count_prefix_with_(
        &self,
        mut xl: usize,
        mut xr: usize,
        y: usize,
        mut f: impl FnMut(usize, Range<usize>, bool),
    ) -> usize {
        if xl == xr || y == 0 {
            return 0;
        } else if y == self.m {
            f(self.lg, xl..xr, false);
            return xr - xl;
        }

        let mut cnt = 0;
        for d in (0..self.lg).rev() {
            let l0 = self.bv[d].count_prefix(xl, false);
            let r0 = self.bv[d].count_prefix(xr, false);
            let l1 = xl + self.mid[d] - l0;
            let r1 = xr + self.mid[d] - r0;
            if y >> d & 1 == 0 {
                (xl, xr) = (l0, r0);
            } else {
                f(d, l0..r0, false);
                cnt += r0 - l0;
                (xl, xr) = (l1, r1);
            }
        }
        cnt
    }
}
