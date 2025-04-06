use algebraic_traits::Monoid;

pub struct FoldableDeque<M: Monoid> {
    fv: Vec<M::Value>,
    bv: Vec<M::Value>,
    fs: Vec<M::Value>,
    bs: Vec<M::Value>,
}

impl<M: Monoid> FromIterator<M::Value> for FoldableDeque<M> {
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
        let mut v = iter.into_iter().collect::<Vec<_>>();
        let len = v.len();
        let mut dq = Self::new();
        for x in v.drain(len / 2..) {
            dq.push_back(x);
        }
        for x in v.drain(..).rev() {
            dq.push_front(x);
        }
        dq
    }
}

impl<M: Monoid> Default for FoldableDeque<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Monoid> FoldableDeque<M> {
    pub fn new() -> Self {
        Self {
            fv: vec![],
            bv: vec![],
            fs: vec![M::unit()],
            bs: vec![M::unit()],
        }
    }

    pub fn from_fn(n: usize, f: impl FnMut(usize) -> M::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }

    pub fn clear(&mut self) {
        self.fv.clear();
        self.bv.clear();
        self.fs.clear();
        self.bs.clear();
        self.fs.push(M::unit());
        self.bs.push(M::unit());
    }

    pub fn len(&self) -> usize {
        self.fv.len() + self.bv.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fv.is_empty() && self.bv.is_empty()
    }

    pub fn push_front(&mut self, x: M::Value) {
        self.fs.push(M::op(&x, self.fs.last().unwrap()));
        self.fv.push(x);
    }

    pub fn push_back(&mut self, x: M::Value) {
        self.bs.push(M::op(self.bs.last().unwrap(), &x));
        self.bv.push(x);
    }

    pub fn pop_front(&mut self) -> Option<M::Value> {
        match self.len() {
            0 => None,
            1 => {
                if self.bv.len() == 1 {
                    self.bs.pop();
                    self.bv.pop()
                } else {
                    self.fs.pop();
                    self.fv.pop()
                }
            }
            _ => {
                if self.fv.is_empty() {
                    self.rebuild();
                }
                self.fs.pop();
                self.fv.pop()
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<M::Value> {
        match self.len() {
            0 => None,
            1 => {
                if self.bv.len() == 1 {
                    self.bs.pop();
                    self.bv.pop()
                } else {
                    self.fs.pop();
                    self.fv.pop()
                }
            }
            _ => {
                if self.bv.is_empty() {
                    self.rebuild();
                }
                self.bs.pop();
                self.bv.pop()
            }
        }
    }

    pub fn front(&self) -> Option<&M::Value> {
        self.fv.last().or(self.bv.first())
    }

    pub fn back(&self) -> Option<&M::Value> {
        self.bv.last().or(self.fv.first())
    }

    pub fn fold_all(&self) -> M::Value {
        M::op(self.fs.last().unwrap(), self.bs.last().unwrap())
    }

    fn rebuild(&mut self) {
        let len = self.len();
        let mut v = self
            .fv
            .drain(..)
            .rev()
            .chain(self.bv.drain(..))
            .collect::<Vec<_>>();
        self.clear();
        for x in v.drain(len / 2..) {
            self.push_back(x);
        }
        for x in v.drain(..).rev() {
            self.push_front(x);
        }
    }
}
