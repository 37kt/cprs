pub fn pentagonal_number_theorem<T>(n: usize) -> Vec<T>
where
    T: Copy + From<i8>,
{
    let mut res = vec![0.into(); n];
    res[0] = 1.into();
    for k in 1.. {
        let i1 = k * (3 * k - 1) / 2;
        let i2 = k * (3 * k + 1) / 2;
        if i1 >= n {
            break;
        }
        let x = if k % 2 == 0 { (-1).into() } else { 1.into() };
        if i1 < n {
            res[i1] = x;
        }
        if i2 < n {
            res[i2] = x;
        }
    }
    res
}
