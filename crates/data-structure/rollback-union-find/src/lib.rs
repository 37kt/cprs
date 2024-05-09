use std::mem::swap;

#[derive(Clone)]
pub struct RollbackUnionFind {
    par: Vec<i32>,
    his: Vec<(usize, i32)>,
    cnt: usize,
}

impl RollbackUnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            his: vec![],
            cnt: n,
        }
    }

    pub fn len(&self) -> usize {
        self.par.len()
    }

    pub fn count(&self) -> usize {
        self.cnt
    }

    pub fn merge(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        self.his.push((x, self.par[x]));
        self.his.push((y, self.par[y]));
        if x == y {
            return false;
        }
        self.cnt -= 1;
        if -self.par[x] < -self.par[y] {
            swap(&mut x, &mut y);
        }
        self.par[x] += self.par[y];
        self.par[y] = x as i32;
        true
    }

    pub fn leader(&self, mut x: usize) -> usize {
        while self.par[x] >= 0 {
            x = self.par[x] as usize;
        }
        x
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    pub fn size(&self, x: usize) -> usize {
        -self.par[self.leader(x)] as usize
    }

    pub fn undo(&mut self) {
        for _ in 0..2 {
            let (x, par) = self.his.pop().unwrap();
            if self.par[x] >= 0 && par < 0 {
                self.cnt += 1;
            }
            self.par[x] = par;
        }
    }

    pub fn groups(&self) -> Vec<Vec<usize>> {
        let mut res = vec![vec![]; self.len()];
        for x in 0..self.len() {
            res[self.leader(x)].push(x);
        }
        res.into_iter().filter(|g| g.len() > 0).collect()
    }
}
