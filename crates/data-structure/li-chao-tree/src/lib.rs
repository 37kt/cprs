use std::{
    mem::swap,
    ops::{Add, Bound, Mul, RangeBounds},
};

#[derive(Clone, Copy)]
struct Line<T>(T, T)
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Ord + Default;

impl<T> Line<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Ord + Default,
{
    fn eval(self, x: T) -> T {
        self.0 * x + self.1
    }

    fn over(self, other: Self, x: T) -> bool {
        self.eval(x) < other.eval(x)
    }
}

pub struct LiChaoTree<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Ord + Default,
{
    n: usize,
    sz: usize,
    xs: Vec<T>,
    seg: Vec<Option<Line<T>>>,
}

impl<T> LiChaoTree<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Ord + Default,
{
    pub fn new(mut xs: Vec<T>) -> Self {
        xs.sort();
        xs.dedup();
        if xs.len() == 0 {
            xs.push(Default::default());
        }
        let sz = 1 << 64 - (xs.len().saturating_sub(1)).leading_zeros() as usize;
        let n = xs.len();
        xs.resize(sz, *xs.last().unwrap());
        Self {
            n,
            sz,
            xs,
            seg: vec![None; sz * 2],
        }
    }

    pub fn add_line(&mut self, (a, b): (T, T)) {
        self.update(Line(a, b), 0, self.sz, 1);
    }

    pub fn add_segment(&mut self, range: impl RangeBounds<T>, (a, b): (T, T)) {
        let mut l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&l) => self.lower_bound(l),
            Bound::Excluded(&l) => self.upper_bound(l),
        } + self.sz;
        let mut r = match range.end_bound() {
            Bound::Unbounded => self.n,
            Bound::Excluded(&r) => self.lower_bound(r),
            Bound::Included(&r) => self.upper_bound(r),
        } + self.sz;
        let line = Line(a, b);
        while l < r {
            if l & 1 != 0 {
                self.update_(line, l);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                self.update_(line, r);
            }
            l >>= 1;
            r >>= 1;
        }
    }

    pub fn min(&self, x: T) -> Option<T> {
        let k = self.xs[0..self.n].binary_search(&x).unwrap();
        self.min_(x, k + self.sz)
    }

    fn min_(&self, x: T, mut k: usize) -> Option<T> {
        let mut res = None;
        while k > 0 {
            res = min(res, self.seg[k].map(|o| o.eval(x)));
            k >>= 1;
        }
        res
    }

    fn update(&mut self, mut line: Line<T>, mut l: usize, mut r: usize, mut k: usize) {
        loop {
            let m = (l + r) >> 1;
            if let Some(o) = self.seg[k].as_mut() {
                let l_over = line.over(*o, self.xs[l]);
                let r_over = line.over(*o, self.xs[r - 1]);
                if l_over == r_over {
                    if l_over {
                        swap(o, &mut line);
                    }
                    return;
                }
                let m_over = line.over(*o, self.xs[m]);
                if m_over {
                    swap(o, &mut line);
                }
                if l_over != m_over {
                    k <<= 1;
                    r = m;
                } else {
                    k = k << 1 | 1;
                    l = m;
                }
            } else {
                self.seg[k] = Some(line);
                return;
            }
        }
    }

    fn update_(&mut self, line: Line<T>, k: usize) {
        let u = 63 - k.leading_zeros();
        let l = (self.sz >> u) * (k - (1 << u));
        let r = l + (self.sz >> u);
        self.update(line, l, r, k);
    }

    fn lower_bound(&self, x: T) -> usize {
        match self.xs[0..self.n].binary_search(&x) {
            Ok(k) => k,
            Err(k) => k,
        }
    }

    fn upper_bound(&self, x: T) -> usize {
        match self.xs[0..self.n].binary_search(&x) {
            Ok(k) => k + 1,
            Err(k) => k,
        }
    }
}

fn min<T>(x: Option<T>, y: Option<T>) -> Option<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Ord,
{
    match (x, y) {
        (Some(x), Some(y)) => Some(x.min(y)),
        (Some(x), None) => Some(x),
        _ => y,
    }
}
