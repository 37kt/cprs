use std::rc::Rc;

#[derive(Clone)]
pub struct PersistentArray<T: Clone, const M: usize> {
    val: Option<Rc<T>>,
    ch: Option<Box<[Rc<Self>]>>,
}

impl<T: Clone, const M: usize> From<Vec<T>> for PersistentArray<T, M> {
    fn from(v: Vec<T>) -> Self {
        let mut res = Self::new();
        for (i, x) in v.into_iter().enumerate() {
            res = res.set(i, x);
        }
        res
    }
}

impl<T: Clone, const M: usize> PersistentArray<T, M> {
    pub fn new() -> Self {
        Self {
            val: None,
            ch: None,
        }
    }

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

    pub fn get(&self, i: usize) -> Option<&T> {
        if i == 0 {
            self.val.as_ref().map(|v| v.as_ref())
        } else {
            self.ch.as_ref().and_then(|ch| ch[i % M].get((i - 1) / M))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut pa = vec![PersistentArray::<_, 8>::new()];
        for i in 1..100 {
            pa.push(pa[0].set(i, 100));
        }
        for i in 0..100 {
            for j in 0..100 {
                if let Some(&x) = pa[i].get(j) {
                    println!("pa[{}][{}] = {}", i, j, x);
                }
            }
        }
    }
}
