// https://noshi91.hatenablog.com/entry/2023/02/18/005856#fn-2ea1eb4f

/// 簡易版 LARSCH (オンライン・ランダムアクセス)  
/// 詳細は上記の記事を参照  
///
/// # 引数
/// * `n`: Monge 下三角行列のサイズ ( (n+1) * (n+1) 行列 )
/// * `f`: `(i, j, x) -> A[i][j]` (i > j)
///   - `x` は `j` 行目の最小値
/// * `init`: `A[0][0]` の値
///
/// # 返り値
/// * `res[i]` = `i` 行目の `(min, argmin)`
pub fn larsch<T: PartialOrd>(
    n: usize,
    mut f: impl FnMut(usize, usize, &T) -> T,
    init: T,
) -> Vec<(T, usize)> {
    let mut min = (0..n + 1).map(|_| (None, 0)).collect::<Vec<_>>();
    min[0] = (Some(init), 0);

    check(n, 0, &mut f, &mut min);
    solve(0, n, &mut f, &mut min);

    min.into_iter().map(|(x, j)| (x.unwrap(), j)).collect()
}

fn check<T: PartialOrd>(
    i: usize,
    j: usize,
    f: &mut impl FnMut(usize, usize, &T) -> T,
    min: &mut [(Option<T>, usize)],
) {
    let x = f(i, j, min[j].0.as_ref().unwrap());
    if min[i].0.is_none() || min[i].0.as_ref().unwrap() > &x {
        min[i] = (Some(x), j);
    }
}

fn solve<T: PartialOrd>(
    l: usize,
    r: usize,
    f: &mut impl FnMut(usize, usize, &T) -> T,
    min: &mut [(Option<T>, usize)],
) {
    if l + 1 >= r {
        return;
    }

    let m = (l + r) / 2;
    for j in min[l].1..=min[r].1 {
        check(m, j, f, min);
    }
    solve(l, m, f, min);
    for j in l + 1..=m {
        check(r, j, f, min);
    }
    solve(m, r, f, min);
}
