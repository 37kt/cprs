use std::ops::RangeBounds;

use graph::Graph;

/// 区間から区間に辺を張るテクニック  
pub struct RangeEdgeGraph<E>
where
    E: Clone + Default,
{
    n: usize,
    m: usize,
    es: Vec<(usize, usize, E)>,
}

impl<E> RangeEdgeGraph<E>
where
    E: Clone + Default,
{
    /// 頂点数 n で初期化する
    pub fn new(n: usize) -> Self {
        let mut s = Self {
            n,
            m: n * 3,
            es: vec![],
        };
        for i in 1..n {
            let l = i << 1 | 0;
            let r = i << 1 | 1;
            s.es.push((s.id(n * 0 + i), s.id(n * 0 + l), E::default()));
            s.es.push((s.id(n * 0 + i), s.id(n * 0 + r), E::default()));
            s.es.push((s.id(n * 2 + l), s.id(n * 2 + i), E::default()));
            s.es.push((s.id(n * 2 + r), s.id(n * 2 + i), E::default()));
        }
        s
    }

    /// 頂点集合 u から頂点集合 v に重み w の辺を張る
    pub fn add_edge(&mut self, u: impl RangeBounds<usize>, v: impl RangeBounds<usize>, w: E) {
        let (mut l1, mut r1) = range_to_pair(u, self.n);
        let (mut l2, mut r2) = range_to_pair(v, self.n);
        l1 += self.n;
        r1 += self.n;
        if l1 == r1 || l2 == r2 {
            return;
        }
        let k = self.m;
        self.m += 1;
        while l1 < r1 {
            if l1 & 1 == 1 {
                self.es.push((self.id(self.n * 2 + l1), k, E::default()));
                l1 += 1;
            }
            if r1 & 1 == 1 {
                r1 -= 1;
                self.es.push((self.id(self.n * 2 + r1), k, E::default()));
            }
            l1 >>= 1;
            r1 >>= 1;
        }
        l2 += self.n;
        r2 += self.n;
        while l2 < r2 {
            if l2 & 1 == 1 {
                self.es.push((k, self.id(l2), w.clone()));
                l2 += 1;
            }
            if r2 & 1 == 1 {
                r2 -= 1;
                self.es.push((k, self.id(r2), w.clone()));
            }
            l2 >>= 1;
            r2 >>= 1;
        }
    }

    /// 超頂点を用いてグラフを構築する  
    /// 元の頂点番号とこのグラフの頂点番号は対応している
    pub fn build(&self) -> Graph<(), E> {
        Graph::from_directed_edges(self.m, &self.es)
    }

    fn id(&self, mut v: usize) -> usize {
        if self.n * 3 <= v {
            v -= self.n * 2;
        }
        if v < self.n {
            v += self.n;
        } else if v < self.n * 2 {
            v -= self.n;
        }
        v
    }
}

fn range_to_pair(i: impl RangeBounds<usize>, n: usize) -> (usize, usize) {
    let start = match i.start_bound() {
        std::ops::Bound::Included(&i) => i,
        std::ops::Bound::Excluded(&i) => i + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let end = match i.end_bound() {
        std::ops::Bound::Included(&i) => i + 1,
        std::ops::Bound::Excluded(&i) => i,
        std::ops::Bound::Unbounded => n,
    };
    (start, end)
}
