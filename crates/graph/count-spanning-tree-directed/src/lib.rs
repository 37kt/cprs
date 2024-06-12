use matrix::Matrix;
use modint::ModInt;

/// r を根とする有向全域木の個数を数える
/// g[i][j] = (i, j) の本数
pub fn count_spanning_tree_directed<M>(g: &[Vec<usize>], r: usize) -> M
where
    M: ModInt,
{
    let n = g.len();
    assert!(g.iter().all(|v| v.len() == n));

    let mut a = Matrix::<M>::from(vec![vec![M::default(); n - 1]; n - 1]);

    let conv = |i: usize| if i < r { i } else { i - 1 };

    for i in 0..n {
        for j in 0..n {
            let ii = conv(i);
            let jj = conv(j);
            if i != r && j != r {
                a[jj][ii] -= g[i][j].into();
            }
            if j != r {
                a[jj][jj] += g[i][j].into();
            }
        }
    }

    a.det()
}
