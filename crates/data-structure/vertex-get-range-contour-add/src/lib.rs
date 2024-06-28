use algebraic::Monoid;
use dual_segment_tree::DualSegmentTree;
use graph::Graph;
use range_contour_query::RangeContourQuery;

pub struct VertexGetRangeContourAdd<M>
where
    M: Monoid,
    M::S: Clone,
{
    rcq: RangeContourQuery,
    seg: Vec<DualSegmentTree<M>>,
}

impl<M> VertexGetRangeContourAdd<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new(a: &[M::S], es: &[(usize, usize)]) -> Self {
        let g = Graph::from_unweighted_undirected_edges(a.len(), es);
        let rcq = RangeContourQuery::new(&g);
        let mut seg = rcq
            .seq
            .iter()
            .map(|v| DualSegmentTree::<M>::new(v.len()))
            .collect::<Vec<_>>();
        for i in 0..a.len() {
            seg[i].apply(0, a[i].clone());
        }
        Self { rcq, seg }
    }

    pub fn get(&self, v: usize) -> M::S {
        let mut res = M::e();
        for (i, j) in self.rcq.point(v) {
            let t = self.seg[i].get(j);
            res = M::op(&res, &t);
        }
        res
    }

    pub fn apply(&mut self, v: usize, x: M::S) {
        self.apply_range(v, 0, 1, x);
    }

    pub fn apply_range(&mut self, v: usize, l: usize, r: usize, x: M::S) {
        for (i, l, r) in self.rcq.range(v, l, r) {
            self.seg[i].apply_range(l..r, x.clone());
        }
    }
}
