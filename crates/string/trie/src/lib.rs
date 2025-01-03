/// Trie + Aho-Corasick  
/// 文字は \[0, SIGMA) の整数とする。
#[derive(Clone)]
pub struct Trie<const SIGMA: usize = 26> {
    children: Vec<[usize; SIGMA]>, // Trie の子ノード
    parent: Vec<usize>,            // Trie の親ノード

    built: bool, // suffix link が構築されているか

    suffix_link: Vec<usize>,   // Aho-Corasick の suffix link
    next: Vec<[usize; SIGMA]>, // Aho-Corasick の next

    words: Vec<usize>,     // 単語に対応する index
    bfs_order: Vec<usize>, // node の BFS 順
    count: Vec<usize>,     // node に対応する文字列の suffix がいくつの単語と一致するか
}

impl<const SIGMA: usize> Trie<SIGMA> {
    /// 空の Trie を作成する。
    pub fn new() -> Self {
        let mut res = Self {
            children: vec![],
            parent: vec![],
            built: false,
            suffix_link: vec![],
            next: vec![],
            words: vec![],
            bfs_order: vec![],
            count: vec![],
        };
        res.add_node();
        res
    }

    /// 単語 s を追加する。  
    /// 追加された単語が対応するノードを返す。  
    /// build が呼ばれていない必要がある。
    pub fn add(&mut self, s: &[usize]) -> usize {
        assert!(!self.built);

        let mut v = 0;
        for &c in s {
            v = self.add_child(v, c);
        }
        self.words.push(v);
        self.count[v] += 1;
        v
    }

    /// suffix link を構築する。  
    /// build が呼ばれていない必要がある。
    pub fn build(&mut self) {
        assert!(!self.built);

        let mut p = 0;
        let mut q = 0;
        self.bfs_order.resize(self.len(), !0);
        self.bfs_order[q] = 0;
        q += 1;
        self.next[0] = [0; SIGMA];
        while p < q {
            let v = self.bfs_order[p];
            p += 1;
            if v != 0 {
                self.next[v] = self.next[self.suffix_link[v]];
            }
            for c in 0..SIGMA {
                let Some(u) = self.child(v, c) else { continue; };
                self.suffix_link[u] = self.next[v][c];
                self.next[v][c] = u;
                self.bfs_order[q] = u;
                q += 1;
            }
        }

        for &v in &self.bfs_order {
            if v != 0 {
                self.count[v] += self.count[self.suffix_link[v]];
            }
        }

        self.built = true;
    }

    /// ノードの数を返す。
    pub fn len(&self) -> usize {
        self.children.len()
    }

    /// ノード v の文字 c に対応する子ノードを返す。  
    /// 存在しない場合は None を返す。
    pub fn child(&self, v: usize, c: usize) -> Option<usize> {
        let res = self.children[v][c];
        if res == !0 {
            None
        } else {
            Some(res)
        }
    }

    /// ノード v の親ノードを返す。  
    /// 根ノードの場合は None を返す。
    pub fn parent(&self, v: usize) -> Option<usize> {
        let res = self.parent[v];
        if res == !0 {
            None
        } else {
            Some(res)
        }
    }

    /// ノード v の suffix link の先のノードを返す。  
    /// 根ノードの場合は None を返す。  
    /// build が呼ばれている必要がある。
    pub fn suffix_link(&self, v: usize) -> Option<usize> {
        assert!(self.built);

        let res = self.suffix_link[v];
        if res == !0 {
            None
        } else {
            Some(res)
        }
    }

    /// ノード v に対応する文字列に文字 c を追加した文字列と
    /// suffix が一致するノードのうち最長のものを返す。  
    /// build が呼ばれている必要がある。
    pub fn next(&self, v: usize, c: usize) -> usize {
        assert!(self.built);
        self.next[v][c]
    }

    /// 追加された単語が対応するノードを返す。
    pub fn words(&self) -> &[usize] {
        &self.words
    }

    /// ノードの BFS 順を返す。  
    /// build が呼ばれている必要がある。
    pub fn bfs_order(&self) -> &[usize] {
        assert!(self.built);
        &self.bfs_order
    }

    /// ノード v に対応する文字列の suffix がいくつの単語と一致するかを返す。  
    /// build が呼ばれている必要がある。
    pub fn count(&self, v: usize) -> usize {
        assert!(self.built);
        self.count[v]
    }

    fn add_node(&mut self) -> usize {
        assert!(!self.built);

        let n = self.children.len();
        self.children.push([!0; SIGMA]);
        self.parent.push(!0);
        self.suffix_link.push(!0);
        self.next.push([!0; SIGMA]);
        self.count.push(0);
        n
    }

    fn add_child(&mut self, v: usize, c: usize) -> usize {
        if let Some(u) = self.child(v, c) {
            u
        } else {
            let u = self.add_node();
            self.children[v][c] = u;
            self.parent[u] = v;
            u
        }
    }
}
