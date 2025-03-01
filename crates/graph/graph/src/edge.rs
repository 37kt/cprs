pub trait Edge {
    fn to(&self) -> usize;
}

pub trait WeightedEdge<T>: Edge {
    fn weight(&self) -> T;
}

impl Edge for usize {
    fn to(&self) -> usize {
        *self
    }
}

impl<T> Edge for (usize, T) {
    fn to(&self) -> usize {
        self.0
    }
}

impl<T> WeightedEdge<T> for (usize, T)
where
    T: Clone,
{
    fn weight(&self) -> T {
        self.1.clone()
    }
}
