const W: u8 = usize::BITS as u8;

#[derive(Clone)]
pub struct BinaryTrie {
    nodes: Vec<Node>,
}

impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    /// x を n 個追加する  
    pub fn insert(&mut self, mut x: usize, n: usize) {
        if n == 0 {
            return;
        }

        let mut v = 0;
        let mut d = W;
        while {
            self.nodes[v].cnt += n;
            d != 0
        } {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            if let Some(link) = self.nodes[v].link[c] {
                let common_prefix = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);
                if common_prefix == link.d {
                    v = link.ch as _;
                    d = common_prefix;
                } else {
                    let w = self.new_node();
                    self.nodes[v].link[c] = Some(Link::new(w, x, common_prefix));
                    let x2 = link.seq & (!0 >> W - common_prefix);
                    let c2 = link.seq >> (common_prefix - 1) & 1;
                    self.nodes[w].link[c2] = Some(Link::new(link.ch as _, x2, link.d));
                    v = w;
                    d = common_prefix;
                }
            } else {
                let u = self.new_node();
                self.nodes[v].link[c] = Some(Link::new(u, x, 0));
                v = u;
                d = 0;
            }
        }
    }

    /// x を n 個削除する  
    /// n 個以上存在しない場合の動作は未定義
    pub fn remove(&mut self, mut x: usize, n: usize) {
        if n == 0 {
            return;
        }

        let mut v = 0;
        let mut d = W;
        while {
            self.nodes[v].cnt -= n;
            d != 0
        } {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            let link = self.nodes[v].link[c].unwrap();
            v = link.ch as _;
            d = link.d;
        }
    }

    /// x の個数を返す  
    pub fn count(&self, mut x: usize) -> usize {
        let mut v = 0;
        let mut d = W;
        while d > 0 {
            x &= !0 >> W - d;
            let c = x >> (d - 1) & 1;
            if let Some(link) = self.nodes[v].link[c] {
                let common_prefix = (W - (x ^ link.seq).leading_zeros() as u8).max(link.d);
                if common_prefix == link.d {
                    v = link.ch as _;
                    d = link.d;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        self.nodes[v].cnt
    }

    fn new_node(&mut self) -> usize {
        self.nodes.push(Node::new());
        self.nodes.len() - 1
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    cnt: usize,
    link: [Option<Link>; 2],
}

impl Node {
    fn new() -> Self {
        Self {
            cnt: 0,
            link: [None; 2],
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Link {
    ch: u32,
    seq: usize,
    d: u8,
    _f: bool,
}

impl Link {
    fn new(ch: usize, seq: usize, d: u8) -> Self {
        Self {
            ch: ch as u32,
            seq,
            d,
            _f: false,
        }
    }
}
