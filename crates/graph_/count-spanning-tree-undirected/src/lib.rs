use matrix::Matrix;
use modint::ModInt;

/// 無向全域木の個数を数える。
/// g[i][j] = (i, j) の本数
pub fn count_spanning_tree_undirected<M>(g: &[Vec<usize>]) -> M
where
    M: ModInt,
{
    let n = g.len();
    assert!(g.iter().all(|v| v.len() == n));

    let mut a = Matrix::<M>::from(vec![vec![M::default(); n - 1]; n - 1]);
    for i in 0..n {
        for j in i + 1..n {
            if i < n - 1 && j < n - 1 {
                a[i][j] -= g[i][j].into();
                a[j][i] -= g[i][j].into();
            }
            if i < n - 1 {
                a[i][i] += g[i][j].into();
            }
            if j < n - 1 {
                a[j][j] += g[i][j].into();
            }
        }
    }

    a.det()
}
