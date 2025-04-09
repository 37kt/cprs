use std::ptr::NonNull;

use crate::operator::Operator;

pub(crate) struct Node<O>
where
    O: Operator,
{
    pub(crate) val: O::X,
    pub(crate) prod: O::P,
    pub(crate) prod_rev: O::P,
    pub(crate) act: O::F,
    pub(crate) par: Option<NonNull<Node<O>>>,
    pub(crate) ch: [Option<NonNull<Node<O>>>; 2],
    pub(crate) len: usize,
    pub(crate) rev: bool,
}

impl<O> Node<O>
where
    O: Operator,
{
    pub(crate) fn new(val: O::X) -> Self {
        Self {
            prod: O::single(&val),
            prod_rev: O::single(&val),
            val,
            act: O::unit_act(),
            par: None,
            ch: [None, None],
            len: 1,
            rev: false,
        }
    }

    pub(crate) fn update(&mut self) {
        self.len = 1;
        self.prod = O::single(&self.val);
        self.prod_rev = O::single(&self.val);
        if let Some(l) = self.ch[0].as_mut() {
            let l = unsafe { l.as_mut() };
            l.push();
            self.len += l.len;
            self.prod = O::op(&l.prod, &self.prod);
            self.prod_rev = O::op(&self.prod_rev, &l.prod_rev);
        }
        if let Some(r) = self.ch[1].as_mut() {
            let r = unsafe { r.as_mut() };
            r.push();
            self.len += r.len;
            self.prod = O::op(&self.prod, &r.prod);
            self.prod_rev = O::op(&r.prod_rev, &self.prod_rev);
        }
    }

    pub(crate) fn push(&mut self) {
        let act = std::mem::replace(&mut self.act, O::unit_act());

        if act != O::unit_act() {
            self.val = O::act_to_val(&self.val, &act);
            self.prod = O::act_to_prod(&self.prod, &act);
            self.prod_rev = O::act_to_prod(&self.prod_rev, &act);
            for mut ch in self.ch {
                if let Some(ch) = ch.as_mut() {
                    let ch = unsafe { ch.as_mut() };
                    ch.act = O::compose(&ch.act, &act);
                }
            }
        }
        if std::mem::take(&mut self.rev) {
            self.ch.swap(0, 1);
            std::mem::swap(&mut self.prod, &mut self.prod_rev);
            for ch in self.ch.iter_mut() {
                if let Some(ch) = ch.as_mut() {
                    let ch = unsafe { ch.as_mut() };
                    ch.rev ^= true;
                }
            }
        }
    }

    pub(crate) fn rotate(&mut self) {
        let mut p = self.par.unwrap();
        let p_ref = unsafe { p.as_mut() };
        let mut g = p_ref.par;
        self.push();

        let dir = match self.pos() {
            Pos::Left => 0,
            Pos::Right => 1,
            Pos::Root => unreachable!(),
        };
        p_ref.ch[dir] = self.ch[dir ^ 1].take();
        if let Some(ch) = p_ref.ch[dir].as_mut() {
            let ch = unsafe { ch.as_mut() };
            ch.par = Some(p);
        }
        self.ch[dir ^ 1] = Some(p);

        p_ref.par = NonNull::new(self);
        self.par = g;
        if let Some(g) = g.as_mut() {
            let g_ref = unsafe { g.as_mut() };
            if Some(p) == g_ref.ch[0] {
                g_ref.ch[0] = NonNull::new(self);
            } else {
                g_ref.ch[1] = NonNull::new(self);
            }
        }
        p_ref.update();
        self.update();
    }

    pub(crate) fn splay(&mut self) {
        while let Some(p) = self.par.as_mut() {
            let p_ref = unsafe { p.as_mut() };
            if p_ref.pos() != Pos::Root {
                if self.pos() == p_ref.pos() {
                    p_ref.rotate();
                } else {
                    self.rotate();
                }
            }
            self.rotate();
        }
    }

    fn pos(&self) -> Pos {
        if let Some(p) = self.par {
            let p_ref = unsafe { p.as_ref() };
            if p_ref.ch[0].map_or(false, |ch| std::ptr::eq(self, ch.as_ptr())) {
                Pos::Left
            } else {
                Pos::Right
            }
        } else {
            Pos::Root
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pos {
    Left,
    Right,
    Root,
}
