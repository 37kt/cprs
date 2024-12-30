/// ビットで表現された集合 s の部分集合を昇順に列挙
pub fn bit_subsets(s: usize) -> impl Iterator<Item = usize> {
    let mut x = 0;
    std::iter::from_fn(move || {
        let res = x;
        if res == !0 {
            None
        } else if x == s {
            x = !0;
            Some(res)
        } else {
            x = x.wrapping_sub(s) & s;
            Some(res)
        }
    })
}
