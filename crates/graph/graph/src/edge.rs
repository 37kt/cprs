pub trait Edge<Weight> {
    fn to(&self) -> usize;
    fn weight(&self) -> &Weight;
}

impl Edge<()> for usize {
    fn to(&self) -> usize {
        *self
    }

    fn weight(&self) -> &() {
        &()
    }
}

impl<Weight> Edge<Weight> for (usize, Weight) {
    fn to(&self) -> usize {
        self.0
    }

    fn weight(&self) -> &Weight {
        &self.1
    }
}
