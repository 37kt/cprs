/// ビットで表現された集合 s の部分集合を昇順に列挙
pub fn bit_subsets(s: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(0usize), move |x| {
        let y = x.wrapping_sub(s) & s;
        if y == 0 {
            None
        } else {
            Some(y)
        }
    })
}
