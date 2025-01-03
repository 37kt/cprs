use std::rc::Rc;

/// 永続配列
#[derive(Clone)]
pub struct PersistentArray<T: Clone, const M: usize = 8> {
    val: Option<Rc<T>>,
    ch: Option<Box<[Rc<Self>]>>,
}

impl<T: Clone, const M: usize> From<Vec<T>> for PersistentArray<T, M> {
    /// Vec から永続配列を構築する。
    fn from(v: Vec<T>) -> Self {
        let mut res = Self::new();
        for (i, x) in v.into_iter().enumerate() {
            res = res.set(i, x);
        }
        res
    }
}

impl<T: Clone, const M: usize> PersistentArray<T, M> {
    /// 空の永続配列を構築する。
    pub fn new() -> Self {
        Self {
            val: None,
            ch: None,
        }
    }

    /// a\[i\] を x に更新する。
    pub fn set(&self, i: usize, x: T) -> Self {
        let mut v = self.clone();
        if i == 0 {
            v.val = Some(Rc::new(x));
            v
        } else {
            if v.ch.is_none() {
                v.ch = Some(vec![Rc::new(Self::new()); M].into_boxed_slice());
            }
            let t = v.ch.as_ref().unwrap()[i % M].set((i - 1) / M, x);
            v.ch.as_mut().unwrap()[i % M] = Rc::new(t);
            v
        }
    }

    /// a\[i\] を取得する。
    pub fn get(&self, i: usize) -> Option<&T> {
        if i == 0 {
            self.val.as_ref().map(|v| v.as_ref())
        } else {
            self.ch.as_ref().and_then(|ch| ch[i % M].get((i - 1) / M))
        }
    }
}
