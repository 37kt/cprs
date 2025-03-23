pub trait BitSubsetExt {
    fn subsets(self) -> BitSubsetIterator;
    fn combinations(self, k: usize) -> BitCombinationIterator;
}

impl BitSubsetExt for usize {
    /// ビット列で表現された集合 s の部分集合を昇順に列挙するイテレータ
    fn subsets(self) -> BitSubsetIterator {
        BitSubsetIterator {
            s: self,
            x: Some(0),
        }
    }

    /// ビット列で表現された集合 s の部分集合の内 popcount が k であるものを昇順に列挙するイテレータ
    fn combinations(self, k: usize) -> BitCombinationIterator {
        BitCombinationIterator {
            s: self,
            x: (k <= self.count_ones() as usize).then_some((1 << k) - 1),
        }
    }
}

#[derive(Clone, Copy)]
pub struct BitSubsetIterator {
    s: usize,
    x: Option<usize>,
}

impl Iterator for BitSubsetIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.x;
        if let Some(x) = res {
            let y = x.wrapping_sub(self.s) & self.s;
            self.x = (y > 0).then_some(y);
        }
        res
    }
}

#[derive(Clone, Copy)]
pub struct BitCombinationIterator {
    s: usize,
    x: Option<usize>,
}

impl Iterator for BitCombinationIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.x;
        if let Some(t) = res {
            if t == 0 {
                self.x = None;
            } else {
                let x = t & t.wrapping_neg();
                let y = t + x;
                let t = (((t & !y) / x) >> 1) | y;
                self.x = (t < 1 << self.s.count_ones() as usize).then_some(t);
            }
        }
        res.map(|t| unsafe { std::arch::x86_64::_pdep_u64(t as u64, self.s as u64) as usize })
    }
}
