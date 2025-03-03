pub trait Mo {
    type Arg;
    type Output;

    /// 区間の端に要素 i を追加する
    #[allow(unused_variables)]
    fn add(&mut self, i: usize) {
        unimplemented!("Please implement either add or both add_left and add_right.");
    }

    /// 区間の端から要素 i を削除する
    #[allow(unused_variables)]
    fn remove(&mut self, i: usize) {
        unimplemented!("Please implement either remove or both remove_left and remove_right.");
    }

    /// 区間の左端に要素 i を追加する  
    /// または (i+1, _) から (i, _) に移動する
    fn add_left(&mut self, i: usize) {
        self.add(i);
    }

    /// 区間の右端に要素 i を追加する  
    /// または (_, i) から (_, i+1) に移動する
    fn add_right(&mut self, i: usize) {
        self.add(i);
    }

    /// 区間の左端から要素 i を削除する  
    /// または (i, _) から (i+1, _) に移動する
    fn remove_left(&mut self, i: usize) {
        self.remove(i);
    }

    /// 区間の右端から要素 i を削除する  
    /// または (_, i+1) から (_, i) に移動する
    fn remove_right(&mut self, i: usize) {
        self.remove(i);
    }

    fn initial_position(&self) -> (usize, usize) {
        (0, 0)
    }

    fn query(&mut self, query: &Self::Arg) -> Self::Output;

    fn solve(&mut self, queries: &[impl Query<Arg = Self::Arg>]) -> Vec<Self::Output> {
        let Some(n) = queries.iter().map(|q| {
            let (l, r) = q.point();
            l.max(r)
        }).max() else {
            return vec![];
        };
        let q = queries.len();
        let w = 1.max((n as f64 / 1.0f64.max((q as f64 * 2.0 / 3.0).sqrt())).round() as usize);

        let mut ord = (0..q).collect::<Vec<_>>();
        ord.sort_unstable_by_key(|&i| {
            let (l, r) = queries[i].point();
            (l / w, if (l / w) & 1 == 0 { r } else { !r })
        });

        let (mut l, mut r) = self.initial_position();
        let mut res = ord
            .iter()
            .map(|&i| {
                let (ql, qr) = queries[i].point();
                while l > ql {
                    l -= 1;
                    self.add_left(l);
                }
                while r < qr {
                    self.add_right(r);
                    r += 1;
                }
                while l < ql {
                    self.remove_left(l);
                    l += 1;
                }
                while r > qr {
                    r -= 1;
                    self.remove_right(r);
                }
                self.query(queries[i].argument())
            })
            .collect::<Vec<_>>();

        for i in 0..q {
            while ord[i] != i {
                let j = ord[i];
                res.swap(i, j);
                ord.swap(i, j);
            }
        }

        res
    }
}

pub trait Query {
    type Arg;

    fn point(&self) -> (usize, usize);
    fn argument(&self) -> &Self::Arg;
}

impl Query for (usize, usize) {
    type Arg = ();

    fn point(&self) -> (usize, usize) {
        *self
    }

    fn argument(&self) -> &Self::Arg {
        &()
    }
}

impl<T> Query for (usize, usize, T) {
    type Arg = T;

    fn point(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn argument(&self) -> &Self::Arg {
        &self.2
    }
}
