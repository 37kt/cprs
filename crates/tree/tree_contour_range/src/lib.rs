// use centroid_decomposition::CentroidDecomposition;
// use csr_array::CsrArray;
// use graph::{Edge, GraphBuilder, UndirectedGraph};

// #[derive(Clone)]
// pub struct TreeContourRange {
//     comp: Vec<usize>,
//     depth: Vec<usize>,
// }

// impl TreeContourRange {
//     pub fn new<W>(g: &CsrArray<impl Edge<W>>) -> Self {
//         todo!()
//     }

//     pub fn len(&self) -> usize {
//         0
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }

// #[test]
// fn test() {
//     use rand::Rng;
//     let mut rng = rand::thread_rng();

//     let mut edges = vec![];
//     let n = 200000;
//     for i in 1..n {
//         let p = rng.gen_range(0..i);
//         edges.push((p, i));
//     }
//     // for i in 0..n / 2 {
//     //     edges.push((i, i + n / 2));
//     //     edges.push((i + n / 2, n));
//     // }
//     let n = n + 1;
//     let g = UndirectedGraph::from_edges(n, edges);
//     let mut cnt = vec![0; n];
//     let mut weight = vec![1; n];
//     CentroidDecomposition::solve(&g, |cd| {
//         cnt[cd.root] += weight[cd.root];
//         for &v in cd.vs {
//             cnt[v] += weight[v];
//         }
//         weight[cd.root] = 0;
//     });

//     cnt.sort_unstable();
//     eprintln!("cnt: {:?}", &cnt[..100]);
//     cnt.reverse();
//     eprintln!("cnt: {:?}", &cnt[..100]);
// }
