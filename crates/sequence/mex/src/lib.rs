/// ナイーブにmexを求める。実験用
pub fn mex(a: &[usize]) -> usize {
    let mut a = a.to_vec();
    a.sort();
    a.dedup();
    for i in 0..a.len() {
        if a[i] != i {
            return i;
        }
    }
    a.len()
}
