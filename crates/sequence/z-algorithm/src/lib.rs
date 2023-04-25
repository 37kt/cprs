pub fn z_algorithm<T>(a: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let mut z = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        let mut k = if j + z[j] <= i {
            0
        } else {
            z[i - j].min(j + z[j] - i)
        };
        while i + k < n && a[k] == a[i + k] {
            k += 1;
        }
        z[i] = k;
        if j + z[j] < i + z[j] {
            j = i;
        }
    }
    z[0] = n;
    z
}