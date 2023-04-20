use algebraic::Monoid;

pub struct SlidingWindowAggregation<M>
where
    M: Monoid,
    M::S: Clone,
{
    fv: Vec<M::S>,
    bv: Vec<M::S>,
    fs: Vec<M::S>,
    bs: Vec<M::S>,
}

impl<M> SlidingWindowAggregation<M>
where
    M: Monoid,
    M::S: Clone,
{
    pub fn new() -> Self {
        Self {
            fv: vec![],
            bv: vec![],
            fs: vec![M::e()],
            bs: vec![M::e()],
        }
    }

    pub fn clear(&mut self) {
        self.fv.clear();
        self.bv.clear();
        self.fs.clear();
        self.fs.push(M::e());
        self.bs.clear();
        self.bs.push(M::e());
    }

    pub fn len(&self) -> usize {
        self.fv.len() + self.bv.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_back(&mut self, x: M::S) {
        self.bs.push(M::op(self.bs.last().unwrap(), &x));
        self.bv.push(x);
    }

    pub fn push_front(&mut self, x: M::S) {
        self.fs.push(M::op(&x, self.fs.last().unwrap()));
        self.fv.push(x);
    }

    pub fn pop_back(&mut self) -> Option<M::S> {
        if self.len() == 0 {
            None
        } else if self.len() == 1 {
            if self.bv.len() == 1 {
                self.bs.pop();
                self.bv.pop()
            } else {
                self.fs.pop();
                self.fv.pop()
            }
        } else {
            if self.bv.is_empty() {
                self.rebuild();
            }
            self.bs.pop();
            self.bv.pop()
        }
    }

    pub fn pop_front(&mut self) -> Option<M::S> {
        if self.len() == 0 {
            None
        } else if self.len() == 1 {
            if self.bv.len() == 1 {
                self.bs.pop();
                self.bv.pop()
            } else {
                self.fs.pop();
                self.fv.pop()
            }
        } else {
            if self.fv.is_empty() {
                self.rebuild();
            }
            self.fs.pop();
            self.fv.pop()
        }
    }

    pub fn prod(&self) -> M::S {
        M::op(self.fs.last().unwrap(), self.bs.last().unwrap())
    }

    fn rebuild(&mut self) {
        let mut v = vec![];
        v.reserve(self.len());
        v.append(&mut self.fv);
        v.reverse();
        v.append(&mut self.bv);
        self.clear();
        for x in v[0..v.len() / 2].iter().rev() {
            self.push_front(x.clone());
        }
        for x in &v[v.len() / 2..v.len()] {
            self.push_back(x.clone());
        }
    }
}
