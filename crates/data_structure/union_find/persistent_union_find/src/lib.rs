use persistent_array::PersistentArray;

const M: usize = 16;

#[derive(Clone, Copy)]
pub struct PersistentUnionFind(PersistentArray<i32, M>);

impl PersistentUnionFind {
    pub fn new(n: usize) -> Self {
        Self(PersistentArray::from_fn(n, |_| -1))
    }

    pub fn root_and_size(&self, mut x: usize) -> (usize, usize) {
        let mut v = self.0[x];
        while v >= 0 {
            x = v as usize;
            v = self.0[x];
        }
        (x, -v as usize)
    }

    pub fn root(&self, x: usize) -> usize {
        self.root_and_size(x).0
    }

    pub fn size(&self, x: usize) -> usize {
        self.root_and_size(x).1
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn merge(&self, x: usize, y: usize) -> (Self, bool) {
        let (mut x, mut xs) = self.root_and_size(x);
        let (mut y, mut ys) = self.root_and_size(y);
        if x == y {
            return (*self, false);
        }
        if xs < ys {
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut xs, &mut ys);
        }
        let new_arr = self.0.set(x, -((xs + ys) as i32)).set(y, x as i32);
        (PersistentUnionFind(new_arr), true)
    }
}
