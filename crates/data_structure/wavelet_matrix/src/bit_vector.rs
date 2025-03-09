use numeric_traits::Integer;

const W: usize = 64;

// すべてのsetを終えた後buildを呼ぶとクエリを投げられるようになる
#[derive(Clone)]
pub(crate) struct BitVector {
    n: usize,
    bit: Vec<u64>,
    sum: Vec<u32>,
}

impl BitVector {
    pub(crate) fn new(n: usize) -> Self {
        let sz = (n + 1).ceil_div(W);
        Self {
            n,
            bit: vec![0; sz],
            sum: vec![0; sz + 1],
        }
    }

    pub(crate) fn set(&mut self, i: usize) {
        assert!(i < self.n);
        self.bit[i / W] |= 1 << i % W;
    }

    pub(crate) fn build(&mut self) {
        for i in 0..self.bit.len() {
            self.sum[i + 1] = self.sum[i] + self.bit[i].count_ones();
        }
    }

    pub(crate) fn count_prefix(&self, i: usize, f: usize) -> usize {
        let cnt = (self.sum[i / W] + (self.bit[i / W] & ((1 << i % W) - 1)).count_ones()) as usize;
        if f == 0 {
            i - cnt
        } else {
            cnt
        }
    }

    pub(crate) fn count(&self, l: usize, r: usize, f: usize) -> usize {
        self.count_prefix(r, f) - self.count_prefix(l, f)
    }
}
