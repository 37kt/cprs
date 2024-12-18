/// n を \[1..n\] で割った商からなる集合の要素を降順に列挙
pub fn quotients(n: usize) -> impl Iterator<Item = usize> {
    let mut i = n;
    std::iter::from_fn(move || {
        if i > 0 {
            let res = i;
            i = n / (n / i + 1);
            Some(res)
        } else {
            None
        }
    })
}
