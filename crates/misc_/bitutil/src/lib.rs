use std::arch::x86_64::{_pdep_u64, _pext_u64};

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

/// ビットで表現された集合 s の部分集合のうち popcount が k であるものを昇順に列挙
pub fn bit_combinations(s: usize, k: usize) -> impl Iterator<Item = usize> {
    let n = popcount(s);
    std::iter::successors((k <= n).then_some((1 << k) - 1), move |&t| {
        if t == 0 {
            return None;
        }
        let x = lsb(t);
        let y = t + x;
        let t = (((t & !y) / x) >> 1) | y;
        if t >= 1 << n {
            None
        } else {
            Some(t)
        }
    })
    .map(move |t| pdep(t, s))
}

/// ビットの立っている数を数える
pub fn popcount(x: usize) -> usize {
    x.count_ones() as usize
}

/// 立っているビットのうち最上位のインデックスを求める
pub fn msb_index(x: usize) -> usize {
    63 - x.leading_zeros() as usize
}

/// 立っているビットのうち最下位のインデックスを求める
pub fn lsb_index(x: usize) -> usize {
    x.trailing_zeros() as usize
}

/// 立っているビットのうち最上位のビットを求める
pub fn msb(x: usize) -> usize {
    1 << msb_index(x)
}

/// 立っているビットのうち最下位のビットを求める
pub fn lsb(x: usize) -> usize {
    x & x.wrapping_neg()
}

/// mask のビットが立っている位置にある x のビットを取り出して下位 popcount(mask) ビットに集める
pub fn pext(x: usize, mask: usize) -> usize {
    unsafe { _pext_u64(x as u64, mask as u64) as usize }
}

/// mask のビットが立っている位置に x の下位 popcount(mask) ビットを配置する
pub fn pdep(x: usize, mask: usize) -> usize {
    unsafe { _pdep_u64(x as u64, mask as u64) as usize }
}
