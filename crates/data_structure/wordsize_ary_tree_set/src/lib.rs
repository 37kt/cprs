// TODO: Iterator ?

use std::ops::Index;

use numeric_traits::Integer;

const W: usize = 64;

#[derive(Clone)]
pub struct WordsizeAryTreeSet {
    n: usize,
    v: Vec<Vec<u64>>,
}

impl WordsizeAryTreeSet {
    pub fn new(mut len: usize) -> Self {
        let n = len;
        let mut v = vec![];
        while {
            len = len.ceil_div(W);
            v.push(vec![0; len]);
            len > 1
        } {}
        Self { n, v }
    }

    /// 保持できる要素数
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.v.last().unwrap()[0] == 0
    }

    pub fn insert(&mut self, mut x: usize) -> bool {
        if self[x] {
            return false;
        }
        for v in &mut self.v {
            v[x / W] |= 1 << (x % W);
            x /= W;
        }
        true
    }

    pub fn remove(&mut self, mut x: usize) -> bool {
        if !self[x] {
            return false;
        }
        for v in &mut self.v {
            v[x / W] &= !(1 << (x % W));
            if v[x / W] != 0 {
                break;
            }
            x /= W;
        }
        true
    }

    pub fn contains(&self, x: usize) -> bool {
        self[x]
    }

    /// x 以上で最小の要素
    pub fn next(&self, mut x: usize) -> Option<usize> {
        let mut d = 0;
        loop {
            let i = x / W;
            let j = x % W;
            let v = self.v.get(d)?.get(i)? & (!0 << j);
            if let Some(to) = v.checked_lsb_index() {
                x = i * W + to;
                if d == 0 {
                    break;
                }
                x *= W;
                d -= 1;
            } else {
                d += 1;
                x = i + 1;
            }
        }
        Some(x)
    }

    /// x 以下で最大の要素
    pub fn prev(&self, mut x: usize) -> Option<usize> {
        x = x.min(self.n.checked_sub(1)?);
        let mut d = 0;
        loop {
            let i = x / W;
            let j = x % W;
            let v = self.v.get(d)?.get(i)? & !(!1 << j);
            if v == 0 {
                d += 1;
                x = i.checked_sub(1)?;
            } else {
                let to = v.msb_index();
                x = i * W + to;
                if d == 0 {
                    break;
                }
                x = (x + 1) * W - 1;
                d -= 1;
            }
        }
        Some(x)
    }
}

impl Index<usize> for WordsizeAryTreeSet {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.n, "index out of bounds");
        let i = index / W;
        let j = index % W;
        if self.v[0][i] >> j & 1 == 1 {
            &true
        } else {
            &false
        }
    }
}
