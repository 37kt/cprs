use crate::HeavyLightDecomposition;

impl HeavyLightDecomposition {
    /// 指定された頂点たちの最小共通祖先関係を保って木を圧縮してできる補助的な木  
    /// `(par, map)` を返す  
    /// `par[i]`: 圧縮後の頂点 `i` の親  
    /// `par[0] == 0`, `i > 0` について `par[i] < i`  
    /// `map[i]`: 圧縮前の頂点たちのうち `i` に対応するもの
    pub fn compress(&self, vs: &[usize]) -> (Vec<usize>, Vec<usize>) {
        if vs.is_empty() {
            return (vec![], vec![]);
        }

        let mut v = Vec::with_capacity(vs.len() * 2 - 1);
        v.extend(vs);
        v.sort_unstable_by_key(|&v| self.down[v]);
        for i in 0..v.len() - 1 {
            v.push(self.lca(v[i], v[i + 1]));
        }
        v.sort_unstable_by_key(|&v| self.down[v]);
        v.dedup();

        let sz = v.len();
        let mut par = vec![0; sz];
        let mut st = vec![0];
        for i in 1..sz {
            while !self.in_subtree(*st.last().unwrap(), v[i]) {
                st.pop();
            }
            par[i] = *st.last().unwrap();
            st.push(i);
        }

        (par, v)
    }
}
