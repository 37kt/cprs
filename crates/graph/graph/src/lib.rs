#[derive(Clone)]
pub struct Graph<V, E>
where
    V: Copy,
    E: Copy,
{
    vs: Vec<V>,
    es: Vec<Vec<(usize, E)>>,
}

impl<V, E> Graph<V, E>
where
    V: Copy + Default,
    E: Copy,
{
    pub fn new(n: usize) -> Self {
        Self {
            vs: vec![Default::default(); n],
            es: vec![vec![]; n],
        }
    }
}

impl<V, E> From<Vec<V>> for Graph<V, E>
where
    V: Copy,
    E: Copy,
{
    fn from(vs: Vec<V>) -> Self {
        Self {
            es: vec![vec![]; vs.len()],
            vs,
        }
    }
}

impl<V, E> Graph<V, E>
where
    V: Copy,
    E: Copy,
{
    pub fn size(&self) -> usize {
        self.vs.len()
    }

    pub fn set_vertex(&mut self, v: usize, w: V) {
        self.vs[v] = w;
    }

    pub fn add_edge(&mut self, u: usize, v: usize, w: E) {
        self.es[u].push((v, w));
    }

    pub fn add_undirected_edge(&mut self, u: usize, v: usize, w: E) {
        self.add_edge(u, v, w);
        self.add_edge(v, u, w);
    }

    pub fn vertex(&self, v: usize) -> V {
        self.vs[v]
    }

    pub fn out_edges(&self, v: usize) -> &Vec<(usize, E)> {
        &self.es[v]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut g = Graph::<i64, i64>::new(100);
        g.add_undirected_edge(0, 1, 100);
    }
}
