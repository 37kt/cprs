/// Cartesian Tree  
/// `par` を返す。
/// `par[i]` は `i` の親を表す。
pub fn cartesian_tree<T>(a: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let n = a.len();
    let mut par = vec![!0; n];
    let mut st = vec![];
    for i in 0..n {
        let mut l = !0;
        while !st.is_empty() && a[*st.last().unwrap()] >= a[i] {
            l = st.pop().unwrap();
        }
        if !st.is_empty() {
            par[i] = st[st.len() - 1];
        }
        if l != !0 {
            par[l] = i;
        }
        st.push(i);
    }
    par
}
