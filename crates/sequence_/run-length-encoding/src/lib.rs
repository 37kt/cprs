/// ランレングス圧縮  
/// (値, 出現回数) のペアの Vec を返す。
pub fn run_length_encoding<T>(a: &[T]) -> Vec<(T, usize)>
where
    T: Clone + Eq,
{
    if a.len() == 0 {
        return vec![];
    }
    let mut res = vec![(a[0].clone(), 0)];
    for x in a {
        if res.last().unwrap().0 == *x {
            res.last_mut().unwrap().1 += 1;
        } else {
            res.push((x.clone(), 1));
        }
    }
    res
}
