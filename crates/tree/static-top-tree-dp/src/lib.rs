use graph::UndirectedGraph;
use heavy_light_decomposition::HeavyLightDecomposition;

#[derive(Clone, Copy)]
enum Type {
    Vertex,
    Compress,
    Rake,
    AddEdge,
    AddVertex,
}

/// Static Top Tree (Fixed root)  
/// 0 を根としている
pub struct StaticTopTree {
    stt_root: usize,
    par: Vec<usize>,
    lch: Vec<usize>,
    rch: Vec<usize>,
    ty: Vec<Type>,
    edge: Vec<usize>,
    par_edge: Vec<usize>,
    child: Vec<usize>,
    cnt: usize,
}

impl StaticTopTree {
    /// 0 を根とする Static Top Tree を構築する
    pub fn new<V: Clone, E: Clone>(g: &UndirectedGraph<V, E>) -> Self {
        let n = g.len();
        let mut s = Self {
            stt_root: !0,
            par: vec![!0; n * 4],
            lch: vec![!0; n * 4],
            rch: vec![!0; n * 4],
            ty: vec![Type::Vertex; n * 4],
            edge: vec![!0; n * 4],
            par_edge: vec![!0; n],
            child: vec![!0; n - 1],
            cnt: n,
        };
        let hld = HeavyLightDecomposition::new(g);
        for v in 0..n {
            for i in 0..g[v].len() {
                let (u, _) = g[v][i];
                if hld.depth[v] < hld.depth[u] {
                    s.par_edge[u] = g.edge_id(v, i);
                    s.child[g.edge_id(v, i)] = u;
                }
            }
        }
        s.stt_root = s.compress(0, g, &hld).0;
        s
    }

    /// 頂点数を返す
    pub fn len(&self) -> usize {
        self.cnt
    }

    fn add(&mut self, mut k: usize, l: usize, r: usize, t: Type) -> usize {
        if k == !0 {
            k = self.cnt;
            self.cnt += 1;
        }
        self.par[k] = !0;
        self.lch[k] = l;
        self.rch[k] = r;
        self.ty[k] = t;
        if l != !0 {
            self.par[l] = k;
        }
        if r != !0 {
            self.par[r] = k;
        }
        k
    }

    fn merge(&mut self, a: &[(usize, usize)], t: Type) -> (usize, usize) {
        if a.len() == 1 {
            return a[0];
        }
        let mut u = a.iter().map(|&(_, s)| s).sum::<usize>();
        let mut m = 0;
        while m < a.len() && a[m].1 < u {
            u -= u.min(a[m].1 * 2);
            m += 1;
        }
        let (i, si) = self.merge(&a[..m], t);
        let (j, sj) = self.merge(&a[m..], t);
        let res = (self.add(!0, i, j, t), si + sj);
        match t {
            Type::Compress => {
                self.edge[res.0] = self.par_edge[a[m].0];
            }
            _ => (),
        }
        res
    }

    fn compress<V: Clone, E: Clone>(
        &mut self,
        mut i: usize,
        g: &UndirectedGraph<V, E>,
        hld: &HeavyLightDecomposition,
    ) -> (usize, usize) {
        let mut chs = vec![self.add_vertex(i, g, hld)];
        while hld.heavy[i] != !0 {
            i = hld.heavy[i];
            chs.push(self.add_vertex(i, g, hld));
        }
        self.merge(&chs, Type::Compress)
    }

    fn rake<V: Clone, E: Clone>(
        &mut self,
        i: usize,
        g: &UndirectedGraph<V, E>,
        hld: &HeavyLightDecomposition,
    ) -> (usize, usize) {
        let mut chs = vec![];
        for &(u, _) in &g[i] {
            if u == hld.par[i] || u == hld.heavy[i] {
                continue;
            }
            chs.push(self.add_edge(u, g, hld));
        }
        if chs.is_empty() {
            (!0, 0)
        } else {
            self.merge(&chs, Type::Rake)
        }
    }

    fn add_edge<V: Clone, E: Clone>(
        &mut self,
        i: usize,
        g: &UndirectedGraph<V, E>,
        hld: &HeavyLightDecomposition,
    ) -> (usize, usize) {
        let (j, sj) = self.compress(i, g, hld);
        let res = (self.add(!0, j, !0, Type::AddEdge), sj);
        self.edge[res.0] = self.par_edge[i];
        res
    }

    fn add_vertex<V: Clone, E: Clone>(
        &mut self,
        i: usize,
        g: &UndirectedGraph<V, E>,
        hld: &HeavyLightDecomposition,
    ) -> (usize, usize) {
        let (j, sj) = self.rake(i, g, hld);
        (
            self.add(
                i,
                j,
                !0,
                if j == !0 {
                    Type::Vertex
                } else {
                    Type::AddVertex
                },
            ),
            sj + 1,
        )
    }
}

#[derive(Clone)]
enum Data<Path, Point> {
    Path(Path),
    Point(Point),
}

impl<Path, Point> Data<Path, Point> {
    fn unwrap_path(&self) -> &Path {
        match self {
            Data::Path(p) => p,
            _ => panic!(),
        }
    }

    fn unwrap_point(&self) -> &Point {
        match self {
            Data::Point(p) => p,
            _ => panic!(),
        }
    }
}

/// 木 dp の各種演算を定義する  
pub trait TreeDPOperator {
    type Path: Clone;
    type Point: Clone;
    type V: Clone;
    type E: Clone;

    /// 頂点 v のみからなる Path Cluster を生成する
    fn vertex(v: &Self::V) -> Self::Path;

    /// 2 つの Path Cluster を Heavy Edge で繋いで 1 つの Path Cluster にする
    fn compress(p: &Self::Path, c: &Self::Path, e: &Self::E) -> Self::Path;

    /// 2 つの Point Cluster の virtual な根を合体させて 1 つの Point Cluster にする
    fn rake(l: &Self::Point, r: &Self::Point) -> Self::Point;

    /// Path Cluster の根と virtual な頂点を Light Edge で繋いで Point Cluster にする
    fn add_edge(d: &Self::Path, e: &Self::E) -> Self::Point;

    /// Point Cluster の根の virtual な頂点を v に置き換えて Path Cluster にする
    fn add_vertex(d: &Self::Point, v: &Self::V) -> Self::Path;
}

pub struct StaticTopTreeDP<O: TreeDPOperator> {
    stt: StaticTopTree,
    sum: Vec<Data<O::Path, O::Point>>,
    vertex: Vec<O::V>,
    edge: Vec<O::E>,
    op: std::marker::PhantomData<O>,
}

impl<O: TreeDPOperator> StaticTopTreeDP<O> {
    /// 0 を根とする Static Top Tree を構築する
    pub fn new(g: &UndirectedGraph<O::V, O::E>) -> Self {
        let stt = StaticTopTree::new(g);
        let mut sum = vec![Data::Path(O::vertex(&g.vertex(0))); stt.len()];
        let vertex = (0..g.len())
            .map(|v| g.vertex(v).clone())
            .collect::<Vec<_>>();
        let mut edge = if g.len() == 1 {
            vec![]
        } else {
            vec![g[0][0].1.clone(); g.len() - 1]
        };
        for v in 0..g.len() {
            sum[v] = Data::Path(O::vertex(&g.vertex(v)));
            for i in 0..g[v].len() {
                edge[g.edge_id(v, i)] = g[v][i].1.clone();
            }
        }
        let mut s = Self {
            stt,
            sum,
            vertex,
            edge,
            op: std::marker::PhantomData,
        };
        s.dfs(s.stt.stt_root);
        s
    }

    /// 0 を根としたときの dp の値を返す
    pub fn prod(&self) -> O::Path {
        self.sum[self.stt.stt_root].unwrap_path().clone()
    }

    /// 頂点 v の値を x に更新する
    pub fn set_vertex(&mut self, mut v: usize, x: O::V) {
        self.vertex[v] = x.clone();
        while v != !0 {
            self.update(v);
            v = self.stt.par[v];
        }
    }

    /// 辺 e の値を x に更新する
    pub fn set_edge(&mut self, e: usize, x: O::E) {
        self.edge[e] = x.clone();
        let mut v = self.stt.child[e];
        while v != !0 {
            self.update(v);
            v = self.stt.par[v];
        }
    }

    fn dfs(&mut self, v: usize) {
        if self.stt.lch[v] != !0 {
            self.dfs(self.stt.lch[v]);
        }
        if self.stt.rch[v] != !0 {
            self.dfs(self.stt.rch[v]);
        }
        self.update(v);
    }

    fn update(&mut self, v: usize) {
        match self.stt.ty[v] {
            Type::Vertex => {
                self.sum[v] = Data::Path(O::vertex(&self.vertex[v]));
            }
            Type::Compress => {
                self.sum[v] = Data::Path(O::compress(
                    self.sum[self.stt.lch[v]].unwrap_path(),
                    self.sum[self.stt.rch[v]].unwrap_path(),
                    &self.edge[self.stt.edge[v]],
                ));
            }
            Type::Rake => {
                self.sum[v] = Data::Point(O::rake(
                    self.sum[self.stt.lch[v]].unwrap_point(),
                    self.sum[self.stt.rch[v]].unwrap_point(),
                ));
            }
            Type::AddEdge => {
                self.sum[v] = Data::Point(O::add_edge(
                    self.sum[self.stt.lch[v]].unwrap_path(),
                    &self.edge[self.stt.edge[v]],
                ));
            }
            Type::AddVertex => {
                self.sum[v] = Data::Path(O::add_vertex(
                    self.sum[self.stt.lch[v]].unwrap_point(),
                    &self.vertex[v],
                ));
            }
        }
    }
}
