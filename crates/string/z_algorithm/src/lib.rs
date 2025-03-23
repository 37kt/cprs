/// `z[i]` = `s[i..]` と `s[..]` の最長共通接頭辞の長さ
pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }
    let mut z = vec![0; n];
    let mut j = 0;
    for i in 1..n {
        let mut k = (j + z[j]).saturating_sub(i).min(z[i - j]);
        while i + k < n && s[k] == s[i + k] {
            k += 1;
        }
        z[i] = k;
        if j + z[j] < i + z[i] {
            j = i;
        }
    }
    z[0] = n;
    z
}
