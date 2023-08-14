use std::mem::swap;

use persistent_array::PersistentArray;

#[derive(Clone)]
pub struct PersistentUnionFind(PersistentArray<i32, 8>);

impl PersistentUnionFind {
    pub fn new(n: usize) -> Self {
        Self(PersistentArray::from(vec![-1; n]))
    }

    pub fn merge(&self, x: usize, y: usize) -> Self {
        let (mut x, mut xs) = self.leader_with_size(x);
        let (mut y, mut ys) = self.leader_with_size(y);
        if x == y {
            return self.clone();
        }
        if xs < ys {
            swap(&mut x, &mut y);
            swap(&mut xs, &mut ys);
        }
        let t = self.0.set(x, -((xs + ys) as i32));
        let t = t.set(y, x as i32);
        Self(t)
    }

    pub fn leader_with_size(&self, x: usize) -> (usize, usize) {
        let t = *self.0.get(x).unwrap();
        if t < 0 {
            (x, -t as usize)
        } else {
            self.leader_with_size(t as usize)
        }
    }

    pub fn leader(&self, x: usize) -> usize {
        self.leader_with_size(x).0
    }

    pub fn size(&self, x: usize) -> usize {
        self.leader_with_size(x).1
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader_with_size(x) == self.leader_with_size(y)
    }
}
