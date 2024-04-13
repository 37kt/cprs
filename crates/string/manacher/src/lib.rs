// reference: https://ngtkana.hatenablog.com/entry/2024/03/17/034026
/// 先頭、文字の間、末尾にダミー文字を入れたときの回文半径を求める
pub fn manacher<T: Eq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut a = vec![0; 2 * n + 1];
    let mut i = 1;
    let mut j = 1;
    while i <= 2 * n {
        while j < i && i + j < 2 * n && s[(i - j) / 2 - 1] == s[(i + j) / 2] {
            j += 2;
        }
        a[i] = j;
        if j == 0 {
            i += 1;
            j = 1;
            continue;
        }
        let mut k = 1;
        while k <= i && k + a[i - k] < j {
            a[i + k] = a[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    a
}
