#[derive(Clone, Copy)]
enum Inner<K, V> {
    Empty,
    One(K, V),
    Two([(K, V); 2]),
}

/// キーが異なる上位高々 2 つの要素を管理する。
#[derive(Clone, Copy)]
pub struct Top2<K, V>
where
    K: Eq + Copy,
    V: Ord + Copy,
{
    inner: Inner<K, V>,
}

impl<K, V> Top2<K, V>
where
    K: Eq + Copy,
    V: Ord + Copy,
{
    /// 要素 0 個で初期化する。
    pub fn new() -> Self {
        Top2 {
            inner: Inner::Empty,
        }
    }

    pub fn push(&mut self, key: K, value: V) {
        match self.inner {
            Inner::Empty => {
                self.inner = Inner::One(key, value);
            }
            Inner::One(k, v) => {
                if k == key {
                    self.inner = Inner::One(k, v.max(value));
                } else {
                    self.inner = Inner::Two([(k, v), (key, value)]);
                }
            }
            Inner::Two([(k1, v1), (k2, v2)]) => {
                if k1 == key {
                    self.inner = Inner::Two([(k1, v1.max(value)), (k2, v2)]);
                } else if k2 == key {
                    self.inner = Inner::Two([(k1, v1), (k2, v2.max(value))]);
                } else if v1 < value {
                    self.inner = Inner::Two([(key, value), (k1, v1)]);
                } else if v2 < value {
                    self.inner = Inner::Two([(k1, v1), (key, value)]);
                }
            }
        }
        match self.inner {
            Inner::Two([(k1, v1), (k2, v2)]) if v1 < v2 => {
                self.inner = Inner::Two([(k2, v2), (k1, v1)]);
            }
            _ => {}
        }
    }

    pub fn append(&mut self, other: &Self) {
        match other.inner {
            Inner::Empty => {}
            Inner::One(k, v) => {
                self.push(k, v);
            }
            Inner::Two([(k1, v1), (k2, v2)]) => {
                self.push(k1, v1);
                self.push(k2, v2);
            }
        }
    }

    /// key 以外の最大値を取得する
    pub fn get_other(&self, key: K) -> Option<V> {
        match self.inner {
            Inner::Empty => None,
            Inner::One(k, v) => {
                if k == key {
                    None
                } else {
                    Some(v)
                }
            }
            Inner::Two([(k1, v1), (_, v2)]) => {
                if k1 == key {
                    Some(v2)
                } else {
                    Some(v1)
                }
            }
        }
    }

    /// 保持しているすべての要素を取得する。
    pub fn get_all(&self) -> Vec<(K, V)> {
        match self.inner {
            Inner::Empty => vec![],
            Inner::One(k, v) => vec![(k, v)],
            Inner::Two([(k1, v1), (k2, v2)]) => vec![(k1, v1), (k2, v2)],
        }
    }
}

impl<K, V> Default for Top2<K, V>
where
    K: Eq + Copy,
    V: Ord + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
