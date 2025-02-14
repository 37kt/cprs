use combination::Combination;
use modint::ModInt;

pub fn lagrange_interpolation<T: ModInt>(ys: &[T], x: usize) -> T {
    let n = ys.len() - 1;
    if x <= n {
        return ys[x];
    }
    let mut res = 0.into();
    let mut dp = vec![1.into(); n + 1];
    let mut pd = vec![1.into(); n + 1];
    for i in 0..n {
        dp[i + 1] = dp[i] * (x - i).into();
    }
    for i in (1..=n).rev() {
        pd[i - 1] = pd[i] * (x - i).into();
    }
    let comb = Combination::<T>::new();
    comb.expand(n);
    for i in 0..=n {
        let tmp = ys[i] * dp[i] * pd[i] * comb.fact_inv(i) * comb.fact_inv(n - i);
        if (n - i) & 1 == 1 {
            res -= tmp;
        } else {
            res += tmp;
        }
    }
    res
}
