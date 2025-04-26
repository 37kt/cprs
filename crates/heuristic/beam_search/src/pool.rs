use crate::index::Index;

pub(crate) struct Pool<T> {
    nodes: Vec<T>,
    free: Vec<Index>,
}

impl<T> Pool<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, node: T) -> Index {
        if let Some(index) = self.free.pop() {
            self.nodes[index.unwrap()] = node;
            index
        } else {
            let i = self.nodes.len();
            self.nodes.push(node);
            Index::from(i)
        }
    }

    pub fn remove(&mut self, index: Index) {
        self.free.push(index);
    }

    pub fn get(&self, index: Index) -> &T {
        &self.nodes[index.unwrap()]
    }

    pub fn get_mut(&mut self, index: Index) -> &mut T {
        &mut self.nodes[index.unwrap()]
    }
}
