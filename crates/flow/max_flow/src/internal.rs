#[derive(Clone, Copy)]
pub(crate) struct Edge {
    pub(crate) dst: usize,
    pub(crate) cap: i64,
}

#[derive(Clone)]
pub(crate) struct Queue {
    data: Vec<usize>,
    head: usize,
    tail: usize,
}

impl Queue {
    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            head: 0,
            tail: 0,
        }
    }

    pub(crate) fn set_capacity(&mut self, cap: usize) {
        self.data.resize(cap, 0);
    }

    pub(crate) fn clear(&mut self) {
        self.head = 0;
        self.tail = 0;
    }

    pub(crate) fn push(&mut self, v: usize) {
        self.data[self.tail] = v;
        self.tail += 1;
    }

    pub(crate) fn pop(&mut self) -> Option<usize> {
        (self.head != self.tail).then(|| {
            self.head += 1;
            self.data[self.head - 1]
        })
    }
}
