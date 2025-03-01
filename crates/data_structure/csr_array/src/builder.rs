use crate::CsrArray;

#[derive(Clone)]
pub struct CsrArrayBuilder<T> {
    pub(crate) n: usize,
    pub(crate) idx: Vec<usize>,
    pub(crate) val: Vec<T>,
}

impl<T> CsrArrayBuilder<T> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            idx: vec![],
            val: vec![],
        }
    }

    pub fn with_capacity(n: usize, capacity: usize) -> Self {
        Self {
            n,
            idx: Vec::with_capacity(capacity),
            val: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, i: usize, x: T) {
        self.idx.push(i);
        self.val.push(x);
    }

    pub fn build(mut self) -> CsrArray<T> {
        let mut sep = vec![0; self.n + 1];
        for &i in &self.idx {
            sep[i] += 1;
        }
        for i in 0..self.n {
            sep[i + 1] += sep[i];
        }
        let mut ord = vec![0; self.idx.len()];
        for (j, &i) in self.idx.iter().enumerate().rev() {
            sep[i] -= 1;
            ord[j] = sep[i];
        }
        for i in 0..self.idx.len() {
            while ord[i] != i {
                let j = ord[i];
                ord.swap(i, j);
                self.val.swap(i, j);
            }
        }
        CsrArray { sep, val: self.val }
    }
}
