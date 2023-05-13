#[derive(Clone)]
pub struct WAryTreeSet {
    m: usize,
    v: Box<[Box<[usize]>]>,
}

impl WAryTreeSet {
    pub fn new(mut n: usize) -> Self {
        let mut v = vec![];
        let mut l = 1;
        while n > 0 {
            n >>= 6;
            v.push(vec![0; l].into_boxed_slice());
            l <<= 6;
        }
        Self {
            m: v.len(),
            v: v.into_boxed_slice(),
        }
    }

    pub fn contains(&self, x: usize) -> bool {
        self.v[self.m - 1][x >> 6] & 1 << (x & 63) != 0
    }

    pub fn insert(&mut self, mut x: usize) -> bool {
        if self.contains(x) {
            false
        } else {
            for v in self.v.iter_mut().rev() {
                v[x >> 6] |= 1 << (x & 63);
                x >>= 6;
            }
            true
        }
    }

    pub fn remove(&mut self, mut x: usize) -> bool {
        if !self.contains(x) {
            false
        } else {
            for i in (0..self.m).rev() {
                if i + 1 == self.m || self.v[i + 1][x] == 0 {
                    self.v[i][x >> 6] ^= 1 << (x & 63);
                }
                x >>= 6;
            }
            true
        }
    }

    pub fn min(&self) -> Option<usize> {
        (self.v[0][0] != 0).then(|| {
            self.v
                .iter()
                .fold(0, |t, v| t << 6 | v[t].trailing_zeros() as usize)
        })
    }

    pub fn max(&self) -> Option<usize> {
        (self.v[0][0] != 0).then(|| {
            self.v
                .iter()
                .fold(0, |t, v| t << 6 | 63 - v[t].leading_zeros() as usize)
        })
    }

    pub fn next(&self, mut x: usize) -> Option<usize> {
        for i in (0..self.m).rev() {
            let mask = if i + 1 == self.m {
                !((1 << (x & 63)) - 1)
            } else {
                !0 << (x & 63) << 1
            };
            let t = (self.v[i][x >> 6] & mask).trailing_zeros() as usize;
            if t != 64 {
                let mut t = x & !63 | t;
                for j in i + 1..self.m {
                    t = t << 6 | self.v[j][t].trailing_zeros() as usize;
                }
                return Some(t);
            }
            x >>= 6;
        }
        None
    }

    pub fn prev(&self, mut x: usize) -> Option<usize> {
        for i in (0..self.m).rev() {
            let mask = if i + 1 == self.m {
                !(!0 << (x & 63) << 1)
            } else {
                (1 << (x & 63)) - 1
            };
            let t = (self.v[i][x >> 6] & mask).leading_zeros() as usize;
            if t != 64 {
                let mut t = x >> 6 << 6 | 63 - t;
                for j in i + 1..self.m {
                    t = t << 6 | 63 - self.v[j][t].leading_zeros() as usize;
                }
                return Some(t);
            }
            x >>= 6;
        }
        None
    }
}
