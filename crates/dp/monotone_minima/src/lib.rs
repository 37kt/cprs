/// h * w 行列 A の各行の argmin が単調増加であるときの argmin を列挙する。  
///
/// # 引数
/// - `h`: 行列の行数。
/// - `w`: 行列の列数。
/// - `select(i, j, k)`: `A[i][j] >= A[i][k]` を返す関数。 (`j < k` が保証されている)
///
/// # 戻り値
/// 各行の argmin
pub fn monotone_minima(
    h: usize,
    w: usize,
    mut select: impl FnMut(usize, usize, usize) -> bool,
) -> Vec<usize> {
    let mut argmin = vec![0; h];
    dfs(0, h, 0, w, &mut select, &mut argmin);
    argmin
}

fn dfs(
    il: usize,
    ir: usize,
    jl: usize,
    jr: usize,
    select: &mut impl FnMut(usize, usize, usize) -> bool,
    argmin: &mut [usize],
) {
    if il >= ir {
        return;
    }

    let im = (il + ir) / 2;
    argmin[im] = jl;
    for j in jl + 1..jr {
        if select(im, argmin[im], j) {
            argmin[im] = j;
        }
    }

    dfs(il, im, jl, argmin[im] + 1, select, argmin);
    dfs(im + 1, ir, argmin[im], jr, select, argmin);
}
