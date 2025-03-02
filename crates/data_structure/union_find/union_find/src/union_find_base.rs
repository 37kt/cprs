use algebraic_traits::{Commutative, Group, Semigroup, Unital};

struct Node<Potential, Aggregate>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    par_sz: i32,
    pot: Potential::Value,
    sum: Aggregate::Value,
}

impl<Potential, Aggregate> Clone for Node<Potential, Aggregate>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            par_sz: self.par_sz,
            pot: self.pot.clone(),
            sum: self.sum.clone(),
        }
    }
}

pub(crate) struct UnionFindBase<Potential, Aggregate, const UNDOABLE: bool>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    nodes: Vec<Node<Potential, Aggregate>>,
    history: Vec<(u32, Node<Potential, Aggregate>)>,
}

impl<Potential, Aggregate, const UNDOABLE: bool> Clone
    for UnionFindBase<Potential, Aggregate, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            history: self.history.clone(),
        }
    }
}

impl<Potential, Aggregate, const UNDOABLE: bool> FromIterator<Aggregate::Value>
    for UnionFindBase<Potential, Aggregate, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = Aggregate::Value>>(iter: T) -> Self {
        let nodes = iter
            .into_iter()
            .map(|sum| Node {
                par_sz: -1,
                pot: Potential::unit(),
                sum,
            })
            .collect();
        Self {
            nodes,
            history: vec![],
        }
    }
}

impl<Potential, Aggregate, const UNDOABLE: bool> UnionFindBase<Potential, Aggregate, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    pub(crate) fn from_fn(n: usize, f: impl FnMut(usize) -> Aggregate::Value) -> Self {
        Self::from_iter((0..n).map(f))
    }
}

impl<Potential, Aggregate, const UNDOABLE: bool> UnionFindBase<Potential, Aggregate, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup + Unital,
    Aggregate::Value: Clone,
{
    pub(crate) fn new(n: usize) -> Self {
        Self::from_fn(n, |_| Aggregate::unit())
    }
}

impl<Potential, Aggregate, const UNDOABLE: bool> UnionFindBase<Potential, Aggregate, UNDOABLE>
where
    Potential: Group,
    Potential::Value: Clone,
    Aggregate: Commutative + Semigroup,
    Aggregate::Value: Clone,
{
    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub(crate) fn merge_with(
        &mut self,
        x: usize,
        y: usize,
        mut d: Potential::Value,
        mut f: impl FnMut(usize, usize),
    ) -> bool {
        let (mut x, px) = self.find(x);
        let (mut y, py) = self.find(y);

        if UNDOABLE {
            self.history.push((x as u32, self.nodes[x].clone()));
            self.history.push((y as u32, self.nodes[y].clone()));
        }

        if x == y {
            return false;
        }

        d = Potential::op(&Potential::op(&px, &d), &Potential::inv(&py));

        if -self.nodes[x].par_sz < -self.nodes[y].par_sz {
            std::mem::swap(&mut x, &mut y);
            d = Potential::inv(&d);
        }

        self.nodes[x].par_sz += self.nodes[y].par_sz;
        self.nodes[x].sum = Aggregate::op(&self.nodes[x].sum, &self.nodes[y].sum);
        self.nodes[y].par_sz = x as i32;
        self.nodes[y].pot = d;

        f(x, y);

        true
    }

    pub(crate) fn find(&mut self, x: usize) -> (usize, Potential::Value) {
        assert!(x < self.nodes.len());

        if UNDOABLE {
            let mut x = x as i32;
            let mut pot = Potential::unit();
            loop {
                let p = self.nodes[x as usize].par_sz;
                if p < 0 {
                    break;
                }
                pot = Potential::op(&self.nodes[x as usize].pot, &pot);
                x = p;
            }
            (x as usize, pot)
        } else {
            let mut x = x as i32;
            let mut pot = Potential::unit();
            loop {
                let p = self.nodes[x as usize].par_sz;
                if p < 0 {
                    break;
                }
                self.nodes[x as usize].pot =
                    Potential::op(&self.nodes[p as usize].pot, &self.nodes[x as usize].pot);
                pot = Potential::op(&self.nodes[x as usize].pot, &pot);
                let pp = self.nodes[p as usize].par_sz;
                if pp >= 0 {
                    self.nodes[x as usize].par_sz = pp;
                    x = pp;
                } else {
                    x = p;
                }
            }
            (x as usize, pot)
        }
    }

    pub(crate) fn root(&mut self, x: usize) -> usize {
        self.find(x).0
    }

    pub(crate) fn potential(&mut self, x: usize) -> Potential::Value {
        self.find(x).1
    }

    pub(crate) fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        -self.nodes[x].par_sz as usize
    }

    pub(crate) fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub(crate) fn diff(&mut self, x: usize, y: usize) -> Option<Potential::Value> {
        let (x, px) = self.find(x);
        let (y, py) = self.find(y);
        (x == y).then_some(Potential::op(&Potential::inv(&px), &py))
    }

    pub(crate) fn component_sum(&mut self, x: usize) -> Aggregate::Value {
        let x = self.root(x);
        self.nodes[x].sum.clone()
    }

    pub(crate) fn set_component_sum(&mut self, x: usize, sum: Aggregate::Value) {
        let x = self.root(x);
        self.nodes[x].sum = sum;
    }

    pub(crate) fn add_component_sum(&mut self, x: usize, sum: Aggregate::Value) {
        let x = self.root(x);
        self.nodes[x].sum = Aggregate::op(&self.nodes[x].sum, &sum);
    }

    pub(crate) fn undo(&mut self) {
        assert!(UNDOABLE);

        for _ in 0..2 {
            let (x, node) = self.history.pop().unwrap();
            self.nodes[x as usize] = node;
        }
    }
}
