use csr_array::CsrArray;

pub trait GraphBuilder<E, G> {
    fn from_edges(n: usize, edges: impl IntoIterator<Item = E>) -> G;
}

pub enum DirectedGraph {}

pub enum UndirectedGraph {}

impl<T> GraphBuilder<(usize, usize, T), CsrArray<(usize, T)>> for DirectedGraph {
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = (usize, usize, T)>,
    ) -> CsrArray<(usize, T)> {
        CsrArray::new(n, edges.into_iter().map(|(a, b, c)| (a, (b, c))))
    }
}

impl<'a, T> GraphBuilder<&'a (usize, usize, T), CsrArray<(usize, T)>> for DirectedGraph
where
    T: 'a + Clone,
{
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = &'a (usize, usize, T)>,
    ) -> CsrArray<(usize, T)> {
        CsrArray::new(n, edges.into_iter().cloned().map(|(a, b, c)| (a, (b, c))))
    }
}

impl GraphBuilder<(usize, usize), CsrArray<usize>> for DirectedGraph {
    fn from_edges(n: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> CsrArray<usize> {
        CsrArray::new(n, edges.into_iter())
    }
}

impl<'a> GraphBuilder<&'a (usize, usize), CsrArray<usize>> for DirectedGraph {
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = &'a (usize, usize)>,
    ) -> CsrArray<usize> {
        CsrArray::new(n, edges.into_iter().cloned())
    }
}

impl<T> GraphBuilder<(usize, usize, T), CsrArray<(usize, T)>> for UndirectedGraph
where
    T: Clone,
{
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = (usize, usize, T)>,
    ) -> CsrArray<(usize, T)> {
        CsrArray::new(
            n,
            edges
                .into_iter()
                .flat_map(|(a, b, c)| [(a, (b, c.clone())), (b, (a, c))]),
        )
    }
}

impl<'a, T> GraphBuilder<&'a (usize, usize, T), CsrArray<(usize, T)>> for UndirectedGraph
where
    T: 'a + Clone,
{
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = &'a (usize, usize, T)>,
    ) -> CsrArray<(usize, T)> {
        CsrArray::new(
            n,
            edges
                .into_iter()
                .cloned()
                .flat_map(|(a, b, c)| [(a, (b, c.clone())), (b, (a, c))]),
        )
    }
}

impl GraphBuilder<(usize, usize), CsrArray<usize>> for UndirectedGraph {
    fn from_edges(n: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> CsrArray<usize> {
        CsrArray::new(n, edges.into_iter().flat_map(|(a, b)| [(a, b), (b, a)]))
    }
}

impl<'a> GraphBuilder<&'a (usize, usize), CsrArray<usize>> for UndirectedGraph {
    fn from_edges(
        n: usize,
        edges: impl IntoIterator<Item = &'a (usize, usize)>,
    ) -> CsrArray<usize> {
        CsrArray::new(n, edges.into_iter().flat_map(|&(a, b)| [(a, b), (b, a)]))
    }
}
