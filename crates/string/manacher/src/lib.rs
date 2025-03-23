// reference: https://ngtkana.hatenablog.com/entry/2024/03/17/034026

/// `s` に含まれる回文の中心は、先頭、文字、文字の間、末尾の 2N + 1 個ある。  
/// それぞれを中心とする最長の回文の長さを返す。
pub fn manacher<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; 2 * n + 1];
    let mut i = 1;
    let mut j = 1;
    while i <= 2 * n {
        while j < i && i + j < 2 * n && s[(i - j) / 2 - 1] == s[(i + j) / 2] {
            j += 2;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            j = 1;
            continue;
        }
        let mut k = 1;
        while k <= i && k + res[i - k] < j {
            res[i + k] = res[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}
