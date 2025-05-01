#[derive(Clone)]
pub struct Trie<const SIGMA: usize = 26> {
    children: Vec<[usize; SIGMA]>,
    parent: Vec<usize>,

    built: bool,

    suffix_link: Vec<usize>,
    next: Vec<[usize; SIGMA]>,

    words: Vec<usize>,
    bfs_order: Vec<usize>,
    count: Vec<usize>,
}

impl<const SIGMA: usize> Default for Trie<SIGMA> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIGMA: usize> Trie<SIGMA> {
    pub fn new() -> Self {
        let mut trie = Self {
            children: vec![],
            parent: vec![],
            built: false,
            suffix_link: vec![],
            next: vec![],
            words: vec![],
            bfs_order: vec![],
            count: vec![],
        };
        trie.add_node();
        trie
    }

    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn add_word(&mut self, s: &[usize]) -> usize {
        assert!(!self.built);

        let mut v = 0;
        for &c in s {
            v = self.add_child(v, c);
        }
        self.words.push(v);
        self.count[v] += 1;
        v
    }

    pub fn build_suffix_links(&mut self) {
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
                let Some(u) = self.child(v, c) else {
                    continue;
                };
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

    pub fn child(&self, v: usize, c: usize) -> Option<usize> {
        let u = self.children[v][c];
        if u == !0 {
            None
        } else {
            Some(u)
        }
    }

    pub fn parent(&self, v: usize) -> Option<usize> {
        let p = self.parent[v];
        if p == !0 {
            None
        } else {
            Some(p)
        }
    }

    pub fn suffix_link(&self, v: usize) -> Option<usize> {
        assert!(self.built);

        let s = self.suffix_link[v];
        if s == !0 {
            None
        } else {
            Some(s)
        }
    }

    pub fn next(&self, v: usize, c: usize) -> usize {
        assert!(self.built);
        self.next[v][c]
    }

    pub fn words(&self) -> &[usize] {
        &self.words
    }

    pub fn bfs_order(&self) -> &[usize] {
        assert!(self.built);
        &self.bfs_order
    }

    pub fn count(&self, v: usize) -> usize {
        assert!(self.built);
        self.count[v]
    }

    pub fn find(&self, s: &[usize]) -> Option<usize> {
        let mut v = 0;
        for &c in s {
            v = self.child(v, c)?;
        }
        Some(v)
    }

    pub fn path(&self, s: &[usize]) -> Option<Vec<usize>> {
        let mut path = vec![0];
        let mut v = 0;
        for &c in s {
            v = self.child(v, c)?;
            path.push(v);
        }
        Some(path)
    }

    fn add_node(&mut self) -> usize {
        assert!(!self.built);

        let n = self.len();
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
