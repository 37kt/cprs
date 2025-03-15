const W: usize = 64;

#[derive(Clone)]
pub struct RadixHeap {
    len: usize,
    last: u64,
    buckets: [Vec<(u64, usize)>; W + 1],
    pos: Vec<(usize, usize)>, // i -> (bucket_id, pos)
}

impl RadixHeap {
    /// index の範囲は [0, n)
    pub fn new(n: usize) -> Self {
        Self {
            len: 0,
            last: 0,
            buckets: std::array::from_fn(|_| vec![]),
            pos: vec![(!0, !0); n],
        }
    }

    /// index の範囲は [0, n)
    pub fn clear(&mut self, n: usize) {
        self.len = 0;
        self.last = 0;
        self.buckets.iter_mut().for_each(|b| b.clear());
        self.pos.fill((!0, !0));
        self.pos.resize(n, (!0, !0));
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// 同じ index で大きいものがあったら上書きする
    pub fn push(&mut self, key: u64, index: usize) {
        assert!(self.last <= key);
        if self.pos[index] == (!0, !0) {
            self.len += 1;
            let bi = id(key ^ self.last);
            self.pos[index] = (bi, self.buckets[bi].len());
            self.buckets[bi].push((key, index));
        } else {
            let (bi, bj) = self.pos[index];
            let (old_key, _) = self.buckets[bi][bj];
            if old_key <= key {
                return;
            }
            let new_bi = id(key ^ self.last);
            if new_bi == bi {
                self.buckets[bi][bj].0 = key;
            } else {
                self.buckets[bi].swap_remove(bj);
                if let Some(&(_, i)) = self.buckets[bi].get(bj) {
                    self.pos[i].1 = bj;
                }

                self.pos[index] = (new_bi, self.buckets[new_bi].len());
                self.buckets[new_bi].push((key, index));
            }
        }
    }

    /// (key, index)
    pub fn pop(&mut self) -> Option<(u64, usize)> {
        if self.len == 0 {
            return None;
        }

        let bi = self.buckets.iter().position(|b| !b.is_empty()).unwrap();
        if bi != 0 {
            self.last = *self.buckets[bi].iter().map(|(key, _)| key).min().unwrap();
            for (key, i) in std::mem::take(&mut self.buckets[bi]) {
                let bj = id(key ^ self.last);
                self.pos[i] = (bj, self.buckets[bj].len());
                self.buckets[bj].push((key, i));
            }
        }
        let (key, i) = self.buckets[0].pop().unwrap();
        self.len -= 1;
        self.pos[i] = (!0, !0);
        Some((key, i))
    }
}

fn id(x: u64) -> usize {
    W - x.leading_zeros() as usize
}
