use std::ops::RangeBounds;

use as_half_open_range::AsHalfOpenRange;
use numeric_traits::Integer;

const BLOCK_SIZE: usize = 16;

#[derive(Clone)]
pub struct RangeMinimumQuery<T> {
    array: Vec<T>,
    large: Vec<Vec<u32>>,
    small: Vec<u16>,
}

impl<T> FromIterator<T> for RangeMinimumQuery<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let xs = iter.into_iter().collect::<Vec<_>>();
        let n = xs.len();
        let block_n = n.floor_div(BLOCK_SIZE);
        let large = if let Some(lg_block_n) = block_n.checked_floor_log2() {
            let mut large = Vec::with_capacity(lg_block_n + 1);
            let level = xs
                .chunks_exact(BLOCK_SIZE)
                .enumerate()
                .map(|(b, v)| {
                    (v.iter()
                        .enumerate()
                        .min_by_key(|&(_, x)| x)
                        .map(|(i, _)| i)
                        .unwrap()
                        + b * BLOCK_SIZE) as u32
                })
                .collect::<Vec<_>>();
            large.push(level);
            for i in 0..lg_block_n {
                let level = large[i]
                    .iter()
                    .zip(&large[i][1 << i..])
                    .map(|(&a, &b)| {
                        if xs[a as usize] <= xs[b as usize] {
                            a
                        } else {
                            b
                        }
                    })
                    .collect::<Vec<_>>();
                large.push(level);
            }
            large
        } else {
            vec![]
        };
        let mut small = Vec::with_capacity(n);
        let mut stack = Vec::<usize>::with_capacity(BLOCK_SIZE);
        let mut bit = 0;
        for i in 0..n {
            if i % BLOCK_SIZE == 0 {
                stack.clear();
                bit = 0;
            }
            while let Some(&j) = stack.last() {
                if xs[j] <= xs[i] {
                    break;
                }
                bit &= !(1 << (j % BLOCK_SIZE));
                stack.pop();
            }
            stack.push(i);
            bit |= 1 << (i % BLOCK_SIZE);
            small.push(bit);
        }

        Self {
            array: xs,
            large,
            small,
        }
    }
}

impl<T> RangeMinimumQuery<T>
where
    T: Ord,
{
    pub fn min(&self, range: impl RangeBounds<usize>) -> Option<&T> {
        let (l, r) = range.as_half_open_range(0, self.array.len());
        if l == r {
            return None;
        }
        let bl = l.ceil_div(BLOCK_SIZE);
        let br = r.floor_div(BLOCK_SIZE);
        let mut res = None;
        if bl > br {
            let i = (self.small[r - 1] & (!0 << (l % BLOCK_SIZE))).lsb_index();
            res = Self::merge(res, Some(&self.array[i + br * BLOCK_SIZE]));
        } else {
            if bl < br {
                let d = (br - bl).floor_log2();
                let level = &self.large[d];
                res = Self::merge(res, Some(&self.array[level[bl] as usize]));
                res = Self::merge(res, Some(&self.array[level[br - (1 << d)] as usize]));
            }
            if l % BLOCK_SIZE != 0 {
                let i = (self.small[bl * BLOCK_SIZE - 1] & (!0 << (l % BLOCK_SIZE))).lsb_index();
                res = Self::merge(res, Some(&self.array[i + (bl - 1) * BLOCK_SIZE]));
            }
            if r % BLOCK_SIZE != 0 {
                let i = self.small[r - 1].lsb_index();
                res = Self::merge(res, Some(&self.array[i + br * BLOCK_SIZE]));
            }
        }
        res
    }

    fn merge<'a>(x: Option<&'a T>, y: Option<&'a T>) -> Option<&'a T> {
        x.into_iter().chain(y.into_iter()).min()
    }
}
