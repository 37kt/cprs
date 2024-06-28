use algebraic::Monoid;
use graph::Graph;
use range_contour_query::RangeContourQuery;
use segment_tree::SegmentTree;

pub struct VertexAddRangeContourSum<M>
where
    M: Monoid,
    M::S: Clone,
{
    rcq: RangeContourQuery,
    seg: Vec<SegmentTree<M>>,
}

impl<M> VertexAddRangeContourSum<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(a: &[M::S], es: &[(usize, usize)]) -> Self {
        let g = Graph::from_unweighted_undirected_edges(a.len(), es);
        let rcq = RangeContourQuery::new(&g);
        let seg = rcq
            .seq
            .iter()
            .map(|v| SegmentTree::<M>::from(v.iter().map(|&i| a[i].clone()).collect::<Vec<_>>()))
            .collect::<Vec<_>>();
        Self { rcq, seg }
    }

    pub fn set(&mut self, v: usize, x: M::S) {
        for (i, j) in self.rcq.point(v) {
            self.seg[i].set(j, x.clone());
        }
    }

    pub fn add(&mut self, v: usize, x: M::S) {
        for (i, j) in self.rcq.point(v) {
            let t = self.seg[i].get(j);
            self.seg[i].set(j, M::op(&t, &x));
        }
    }

    pub fn get(&self, v: usize) -> M::S {
        self.seg[v].get(0)
    }

    pub fn prod(&self, v: usize, l: usize, r: usize) -> M::S {
        let mut res = M::e();
        for (i, l, r) in self.rcq.range(v, l, r) {
            let t = self.seg[i].prod(l..r);
            res = M::op(&res, &t);
        }
        res
    }
}
