use algebraic::Monoid;
use dual_segment_tree::DualSegmentTree;
use graph::UndirectedGraph;
use range_contour_query::RangeContourQuery;

/// 等高線加算、頂点値取得クエリ
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
    /// 頂点の値を a 、辺を es で初期化
    pub fn new(a: &[M::S], es: &[(usize, usize)]) -> Self {
        let g = UndirectedGraph::from_unweighted_edges(a.len(), es);
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

    /// 頂点 v の値を取得
    pub fn get(&self, v: usize) -> M::S {
        let mut res = M::e();
        for (i, j) in self.rcq.point(v) {
            let t = self.seg[i].get(j);
            res = M::op(&res, &t);
        }
        res
    }

    /// 頂点 v に x を加算
    pub fn apply(&mut self, v: usize, x: M::S) {
        self.apply_range(v, 0, 1, x);
    }

    /// 頂点 v からの距離が l 以上 r 未満の頂点に x を加算
    pub fn apply_range(&mut self, v: usize, l: usize, r: usize, x: M::S) {
        for (i, l, r) in self.rcq.range(v, l, r) {
            self.seg[i].apply_range(l..r, x.clone());
        }
    }
}
