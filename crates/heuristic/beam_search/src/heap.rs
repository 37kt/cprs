use std::collections::BinaryHeap;

use crate::{candidate::Candidate, index::Index};

pub(crate) struct Heap<Action> {
    capacity: usize,
    heap: BinaryHeap<Candidate<Action>>,
}

impl<Action> Heap<Action> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            heap: BinaryHeap::with_capacity(capacity),
        }
    }

    // 削除される候補の親を返す
    pub fn push(&mut self, x: Candidate<Action>) -> Index {
        if self.heap.len() < self.capacity {
            self.heap.push(x);
            None.into()
        } else if let Some(mut top) = self.heap.peek_mut() {
            if top.score < x.score {
                let parent = top.parent;
                *top = x;
                parent
            } else {
                x.parent
            }
        } else {
            unreachable!()
        }
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Candidate<Action>> + '_ {
        self.heap.drain()
    }
}
