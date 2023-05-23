use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct IntervalSet<T: Clone + Ord>(BTreeMap<T, T>);

impl<T: Clone + Ord> IntervalSet<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn insert(&mut self, mut l: T, mut r: T) {
        assert!(l <= r);
        if l == r {
            return;
        }

        if let Some((a, b)) = self.0.range(..=&l).next_back() {
            if &l <= b {
                l = a.clone();
            }
        }
        for (a, b) in self
            .0
            .range(&l..=&r)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect::<Vec<_>>()
        {
            self.0.remove(&a);
            r = r.max(b);
        }
        self.0.insert(l, r);
    }

    pub fn mex(&self, lower: T) -> T {
        if let Some((_, r)) = self.0.range(..=&lower).next_back() {
            lower.max(r.clone())
        } else {
            lower
        }
    }

    pub fn contains(&self, x: T) -> bool {
        if let Some((_, r)) = self.0.range(..=&x).next_back() {
            &x < r
        } else {
            false
        }
    }

    pub fn intervals(&self) -> impl Iterator<Item = (T, T)> {
        self.0.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut set = IntervalSet::new();
        set.insert(10, 20);
        set.insert(20, 30);
        set.insert(0, 10);
        set.insert(31, 40);
        set.insert(30, 31);
        set.insert(-1, 100);
        set.insert(-1000, -1000);
        dbg!(&set);
    }
}
