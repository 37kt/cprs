/// a に含まれない最小の非負整数を求める。
pub fn mex(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    for i in 0..a.len() {
        while i < a.len() && a[i] != i {
            if a[i] >= a.len() || a[i] == a[a[i]] {
                a.swap_remove(i);
            } else {
                let j = a[i];
                a.swap(i, j);
            }
        }
    }
    a.iter()
        .enumerate()
        .position(|(i, &x)| x != i)
        .unwrap_or(a.len())
}
