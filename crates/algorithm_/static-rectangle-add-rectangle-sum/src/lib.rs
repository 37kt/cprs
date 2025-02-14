use std::{
    mem::swap,
    ops::{Add, Mul, Neg, Sub},
};

/// 二次元平面上の矩形領域に対する加算・和クエリを処理する
///
/// # 概要
/// - すべての矩形加算クエリを処理した後、すべての矩形和クエリを処理する
///
/// # 引数
/// - `add`: 加算クエリの配列 `(xl, xr, yl, yr, w)`
///   - 領域 `[xl, xr) × [yl, yr)` のすべての要素に `w` を加算
/// - `sum`: 和クエリの配列 `(xl, xr, yl, yr)`
///   - 領域 `[xl, xr) × [yl, yr)` の要素の総和を計算
///
/// # 計算量
/// - O(N log N + Q log N)
///   - N: 加算クエリの数
///   - Q: 和クエリの数
pub fn static_rectangle_add_rectangle_sum<I, T>(
    add: &[(I, I, I, I, T)],
    sum: &[(I, I, I, I)],
) -> Vec<T>
where
    I: Copy + TryInto<i64>,
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<i64, Output = T> + Neg<Output = T>,
{
    let cast = |x: I| unsafe { x.try_into().unwrap_unchecked() };

    let mut ys: Vec<i64> = vec![];
    for &(_, _, yl, yr, _) in add {
        ys.push(cast(yl));
        ys.push(cast(yr));
    }
    ys.sort();
    ys.dedup();

    let bs = |y: i64| match ys.binary_search(&y) {
        Ok(i) => i,
        Err(i) => i,
    };

    let mut evs = vec![];
    for i in 0..sum.len() {
        let (xl, xr, _, _) = sum[i];
        evs.push((cast(xl), 0, i));
        evs.push((cast(xr), 1, i));
    }
    for i in 0..add.len() {
        let (xl, xr, _, _, _) = add[i];
        evs.push((cast(xl), 2, i));
        evs.push((cast(xr), 3, i));
    }
    evs.sort();

    let mut fxy = FenwickTree::new(ys.len());
    let mut fx = FenwickTree::new(ys.len());
    let mut fy = FenwickTree::new(ys.len());
    let mut f = FenwickTree::new(ys.len());
    let mut res = vec![T::default(); sum.len()];
    for (x, t, q) in evs {
        if t & 2 != 0 {
            let (_, _, mut yl, mut yr, w) = add[q];
            let mut i = bs(cast(yl));
            let mut j = bs(cast(yr));
            if t & 1 != 0 {
                swap(&mut i, &mut j);
                swap(&mut yl, &mut yr);
            }
            fxy.add(i, w * x * cast(yl));
            fxy.add(j, -w * x * cast(yr));
            fx.add(i, -w * x);
            fx.add(j, w * x);
            fy.add(i, -w * cast(yl));
            fy.add(j, w * cast(yr));
            f.add(i, w);
            f.add(j, -w);
        } else {
            let (_, _, mut yl, mut yr) = sum[q];
            let mut i = bs(cast(yl));
            let mut j = bs(cast(yr));
            if t & 1 != 0 {
                swap(&mut i, &mut j);
                swap(&mut yl, &mut yr);
            }
            res[q] = res[q]
                + fxy.accum(i)
                + fx.accum(i) * cast(yl)
                + fy.accum(i) * x
                + f.accum(i) * x * cast(yl);
            res[q] = res[q]
                - fxy.accum(j)
                - fx.accum(j) * cast(yr)
                - fy.accum(j) * x
                - f.accum(j) * x * cast(yr);
        }
    }

    res
}

struct FenwickTree<T>
where
    T: Copy + Default + Add<Output = T>,
{
    v: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Copy + Default + Add<Output = T>,
{
    fn new(n: usize) -> Self {
        FenwickTree {
            v: vec![T::default(); n],
        }
    }

    fn add(&mut self, mut k: usize, x: T) {
        assert!(k <= self.v.len());
        k += 1;
        while k <= self.v.len() {
            self.v[k - 1] = self.v[k - 1] + x;
            k += k & k.wrapping_neg();
        }
    }

    fn accum(&self, mut k: usize) -> T {
        let mut res = T::default();
        while k > 0 {
            res = res + self.v[k - 1];
            k &= k - 1;
        }
        res
    }
}
