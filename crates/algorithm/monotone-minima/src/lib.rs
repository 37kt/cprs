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

/// select(i, j, k): f(i, j) >= f(i, k)
pub fn monotone_minima(
    h: usize,
    w: usize,
    select: impl Fn(usize, usize, usize) -> bool,
) -> Vec<usize> {
    let mut res = vec![0; h];
    dfs(0, h, 0, w, &select, &mut res);
    res
}
