fn dfs(
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
    select: &impl Fn(usize, usize, usize) -> bool,
    res: &mut [usize],
) {
    if x1 == x2 {
        return;
    }
    let x = (x1 + x2) / 2;
    let mut best_y = y1;
    for y in y1 + 1..y2 {
        if select(x, best_y, y) {
            best_y = y;
        }
    }
    res[x] = best_y;
    dfs(x1, x, y1, best_y + 1, select, res);
    dfs(x + 1, x2, best_y, y2, select, res);
}

/// monotoneな行列に対する最小値の列を求める
///
/// # 概要
/// 行列 A が monotone であるとき、各行について最小値を取る列を効率的に計算する
///
/// # 引数
/// - `h`: 行列の行数
/// - `w`: 行列の列数
/// - `select`: 比較関数。以下の条件を満たす必要がある
///   - `select(i, j, k)` := `A[i][j] ≥ A[i][k]`
///
/// # 戻り値
/// - 長さ `h` のベクトル
/// - `res[i]` は `i` 行目で最小値を取る列のインデックス
///
/// # 計算量
/// - O(TODO)
///   - h: 行数
///   - w: 列数
pub fn monotone_minima(
    h: usize,
    w: usize,
    select: impl Fn(usize, usize, usize) -> bool,
) -> Vec<usize> {
    let mut res = vec![0; h];
    dfs(0, h, 0, w, &select, &mut res);
    res
}
