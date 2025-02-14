use algebraic::Monoid;

/// モノイド総積を持つ VecDeque
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
    /// 空の列で初期化する。
    pub fn new() -> Self {
        Self {
            fv: vec![],
            bv: vec![],
            fs: vec![M::e()],
            bs: vec![M::e()],
        }
    }

    /// 列をクリアする。
    pub fn clear(&mut self) {
        self.fv.clear();
        self.bv.clear();
        self.fs.clear();
        self.fs.push(M::e());
        self.bs.clear();
        self.bs.push(M::e());
    }

    /// 列の長さを取得する。
    pub fn len(&self) -> usize {
        self.fv.len() + self.bv.len()
    }

    /// 列が空かどうかを判定する。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 列の末尾に要素を追加する。
    pub fn push_back(&mut self, x: M::S) {
        self.bs.push(M::op(self.bs.last().unwrap(), &x));
        self.bv.push(x);
    }

    /// 列の先頭に要素を追加する。
    pub fn push_front(&mut self, x: M::S) {
        self.fs.push(M::op(&x, self.fs.last().unwrap()));
        self.fv.push(x);
    }

    /// 列の末尾の要素を取り出す。
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

    /// 列の先頭の要素を取り出す。
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

    /// 列の先頭の要素を取得する。
    pub fn front(&self) -> Option<&M::S> {
        self.fv.first()
    }

    /// 列の末尾の要素を取得する。
    pub fn back(&self) -> Option<&M::S> {
        self.bv.last()
    }

    /// 列の総積を取得する。
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
