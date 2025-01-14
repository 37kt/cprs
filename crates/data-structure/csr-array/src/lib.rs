use std::ops::Index;

/// CSR 形式の二次元配列
#[derive(Clone)]
pub struct CSRArray<T> {
    pos: Vec<usize>,
    data: Vec<T>,
}

impl<T: Clone> CSRArray<T> {
    /// a の各要素 (i, x) について、 i 番目の配列に x が格納される
    pub fn new(n: usize, a: &[(usize, T)]) -> Self {
        let mut pos = vec![0; n + 1];
        for &(i, _) in a {
            pos[i] += 1;
        }
        for i in 0..n {
            pos[i + 1] += pos[i];
        }
        let mut ord = vec![0; a.len()];
        for j in (0..a.len()).rev() {
            let (i, _) = a[j];
            pos[i] -= 1;
            ord[pos[i]] = j;
        }
        let data = ord.into_iter().map(|i| a[i].1.clone()).collect();
        Self { pos, data }
    }
}

impl<T> CSRArray<T> {
    pub fn len(&self) -> usize {
        self.pos.len() - 1
    }

    pub fn iter(&self) -> impl Iterator<Item = &[T]> {
        (0..self.len()).map(|i| &self[i])
    }
}

impl<T> Index<usize> for CSRArray<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        let start = self.pos[i];
        let end = self.pos[i + 1];
        &self.data[start..end]
    }
}
