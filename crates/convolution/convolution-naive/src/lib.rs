use modint::ModInt;

/// 畳み込みを計算する
///
/// # 引数
/// - `a`: 1つ目の配列
/// - `b`: 2つ目の配列
///
/// # 戻り値
/// - 畳み込みの結果（長さは `a.len() + b.len() - 1`）
///
/// # 計算量
/// - O(NM)
///   - N: a.len()
///   - M: b.len()
pub fn convolution_naive<T: ModInt>(a: &[T], b: &[T]) -> Vec<T> {
    let n = a.len();
    let m = b.len();
    if n == 0 || m == 0 {
        return vec![];
    }
    let l = n + m - 1;
    let mut c = vec![0.into(); l];
    if n > m {
        for i in 0..n {
            for j in 0..m {
                c[i + j] += a[i] * b[j];
            }
        }
    } else {
        for j in 0..m {
            for i in 0..n {
                c[i + j] += a[i] * b[j];
            }
        }
    }
    c
}
