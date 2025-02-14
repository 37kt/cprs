use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};

use algebraic::{One, Zero};

pub trait DeterministicFiniteAutoMaton {
    type Symbol;
    type State;

    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>;
    fn is_accepting(&self, state: &Self::State) -> bool;

    #[allow(unused_variables)]
    fn state_to_index(&self, state: &Self::State) -> usize {
        unimplemented!()
    }

    fn state_num(&self) -> usize {
        unimplemented!()
    }

    fn count_accepting_hashmap<T>(
        &self,
        alphabet: impl Iterator<Item = Self::Symbol> + Clone,
        len: usize,
    ) -> T
    where
        Self::State: Eq + std::hash::Hash,
        T: Zero + One + std::ops::Add<Output = T> + Clone,
    {
        self.dp_hashmap(
            alphabet,
            len,
            |_, x| x.clone(),
            |x, y| x.clone() + y.clone(),
            T::zero(),
            T::one(),
        )
    }

    fn count_accepting_vec<T>(
        &self,
        alphabet: impl Iterator<Item = Self::Symbol> + Clone,
        len: usize,
    ) -> T
    where
        T: Zero + One + std::ops::Add<Output = T> + Clone,
    {
        self.dp_vec(
            alphabet,
            len,
            |_, x| x.clone(),
            |x, y| x.clone() + y.clone(),
            T::zero(),
            T::one(),
        )
    }

    fn dp_hashmap<T>(
        &self,
        alphabet: impl Iterator<Item = Self::Symbol> + Clone,
        len: usize,
        act: impl Fn(&Self::Symbol, &T) -> T,
        op: impl Fn(&T, &T) -> T,
        e: T,
        dp_initial: T,
    ) -> T
    where
        Self::State: Eq + std::hash::Hash,
        T: Clone,
    {
        let mut dp = HashMap::<Self::State, T>::new();
        let mut ndp = HashMap::<Self::State, T>::new();
        dp.insert(self.initial_state(), dp_initial);

        for _ in 0..len {
            for (st, val) in dp.drain() {
                for c in alphabet.clone() {
                    let Some(nst) = self.transition(&st, &c) else { continue; };
                    let nval = act(&c, &val);
                    ndp.entry(nst)
                        .and_modify(|sum| *sum = op(sum, &nval))
                        .or_insert(nval);
                }
            }
            std::mem::swap(&mut dp, &mut ndp);
        }

        dp.into_iter()
            .filter(|(st, _)| self.is_accepting(st))
            .fold(e, |acc, (_, val)| op(&acc, &val))
    }

    fn dp_vec<T>(
        &self,
        alphabet: impl Iterator<Item = Self::Symbol> + Clone,
        len: usize,
        act: impl Fn(&Self::Symbol, &T) -> T,
        op: impl Fn(&T, &T) -> T,
        e: T,
        dp_initial: T,
    ) -> T
    where
        T: Clone,
    {
        let mut dp = (0..self.state_num()).map(|_| None).collect::<Vec<_>>();
        dp[self.state_to_index(&self.initial_state())] = Some((self.initial_state(), dp_initial));

        for _ in 0..len {
            let mut ndp = (0..self.state_num()).map(|_| None).collect::<Vec<_>>();
            for (st, val) in dp.into_iter().flatten() {
                for c in alphabet.clone() {
                    let Some(nst) = self.transition(&st, &c) else { continue; };
                    let nval = act(&c, &val);
                    let i = self.state_to_index(&nst);
                    if let Some((_, sum)) = ndp[i].as_mut() {
                        *sum = op(sum, &nval);
                    } else {
                        ndp[i] = Some((nst, nval));
                    }
                }
            }
            dp = ndp;
        }

        dp.into_iter()
            .flatten()
            .filter(|(st, _)| self.is_accepting(st))
            .fold(e, |acc, (_, val)| op(&acc, &val))
    }
}

#[derive(Clone, Copy)]
pub struct NotDFA<A>
where
    A: DeterministicFiniteAutoMaton,
{
    dfa: A,
}

impl<A> DeterministicFiniteAutoMaton for NotDFA<A>
where
    A: DeterministicFiniteAutoMaton,
{
    type Symbol = A::Symbol;
    type State = A::State;

    fn initial_state(&self) -> Self::State {
        self.dfa.initial_state()
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        self.dfa.transition(state, c)
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        !self.dfa.is_accepting(state)
    }

    fn state_to_index(&self, state: &Self::State) -> usize {
        self.dfa.state_to_index(state)
    }

    fn state_num(&self) -> usize {
        self.dfa.state_num()
    }
}

#[derive(Clone, Copy)]
pub struct OperationDFA<A0, A1, Op>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
    Op: Fn(bool, bool) -> bool,
{
    dfa0: A0,
    dfa1: A1,
    op: Op,
}

impl<A0, A1, Op> DeterministicFiniteAutoMaton for OperationDFA<A0, A1, Op>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
    Op: Fn(bool, bool) -> bool,
{
    type Symbol = A0::Symbol;
    type State = (A0::State, A1::State);

    fn initial_state(&self) -> Self::State {
        (self.dfa0.initial_state(), self.dfa1.initial_state())
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        let (st0, st1) = state;
        let st0 = self.dfa0.transition(st0, c)?;
        let st1 = self.dfa1.transition(st1, c)?;
        Some((st0, st1))
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        let (st0, st1) = state;
        (self.op)(self.dfa0.is_accepting(st0), self.dfa1.is_accepting(st1))
    }

    fn state_to_index(&self, state: &Self::State) -> usize {
        let (st0, st1) = state;
        self.dfa0.state_to_index(st0) * self.dfa1.state_num() + self.dfa1.state_to_index(st1)
    }

    fn state_num(&self) -> usize {
        self.dfa0.state_num() * self.dfa1.state_num()
    }
}

impl<A0, A1, Op> OperationDFA<A0, A1, Op>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
    Op: Fn(bool, bool) -> bool,
{
    pub fn new(dfa0: A0, dfa1: A1, op: Op) -> Self {
        Self { dfa0, dfa1, op }
    }
}

impl<A0, A1> OperationDFA<A0, A1, fn(bool, bool) -> bool>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
{
    pub fn or(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| a || b,
        }
    }

    pub fn and(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| a && b,
        }
    }

    pub fn xor(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| a ^ b,
        }
    }

    pub fn nand(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| !(a && b),
        }
    }

    pub fn nor(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| !(a || b),
        }
    }

    pub fn xnor(dfa0: A0, dfa1: A1) -> Self {
        Self {
            dfa0,
            dfa1,
            op: |a, b| !(a ^ b),
        }
    }
}

#[derive(Clone, Copy)]
pub struct LexicographicalDFA<'a, T> {
    seq: &'a [T],
    ord: Ordering,
    eq: bool,
}

impl<'a, T> DeterministicFiniteAutoMaton for LexicographicalDFA<'a, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = (usize, bool);

    fn initial_state(&self) -> Self::State {
        (0, true)
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        let &(i, tight) = state;
        let x = self.seq.get(i)?;
        match (tight, c.cmp(x)) {
            (true, Ordering::Equal) => Some((i + 1, true)),
            (true, ord) if ord == self.ord.reverse() => None,
            _ => Some((i + 1, false)),
        }
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.eq || !state.1
    }

    fn state_to_index(&self, state: &Self::State) -> usize {
        state.1 as usize
    }

    fn state_num(&self) -> usize {
        2
    }
}

impl<'a, T> LexicographicalDFA<'a, T>
where
    T: Ord,
{
    pub fn less_than(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Less,
            eq: false,
        }
    }

    pub fn less_than_equal(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Less,
            eq: true,
        }
    }

    pub fn greater_than(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Greater,
            eq: false,
        }
    }

    pub fn greater_than_equal(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Greater,
            eq: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ReversedLexicographicalDFA<'a, T> {
    seq: &'a [T],
    ord: Ordering,
    eq: bool,
}

impl<'a, T> DeterministicFiniteAutoMaton for ReversedLexicographicalDFA<'a, T>
where
    T: Ord,
{
    type Symbol = T;
    type State = (usize, bool);

    fn initial_state(&self) -> Self::State {
        (self.seq.len(), self.eq)
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        let &(i, accepting) = state;
        let i = i.checked_sub(1)?;
        let x = self.seq.get(i)?;
        match c.cmp(x) {
            Ordering::Equal => Some((i, accepting)),
            ord => Some((i, ord == self.ord)),
        }
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        state.1
    }

    fn state_to_index(&self, state: &Self::State) -> usize {
        state.1 as usize
    }

    fn state_num(&self) -> usize {
        2
    }
}

impl<'a, T> ReversedLexicographicalDFA<'a, T>
where
    T: Ord,
{
    pub fn less_than(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Less,
            eq: false,
        }
    }

    pub fn less_than_equal(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Less,
            eq: true,
        }
    }

    pub fn greater_than(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Greater,
            eq: false,
        }
    }

    pub fn greater_than_equal(seq: &'a [T]) -> Self {
        Self {
            seq,
            ord: Ordering::Greater,
            eq: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SymbolMapDFA<A, S, F>
where
    A: DeterministicFiniteAutoMaton,
    F: Fn(&S) -> A::Symbol,
{
    dfa: A,
    map: F,
    _marker: PhantomData<fn() -> S>,
}

impl<A, S, F> SymbolMapDFA<A, S, F>
where
    A: DeterministicFiniteAutoMaton<Symbol = S>,
    F: Fn(&S) -> A::Symbol,
{
    pub fn new(dfa: A, map: F) -> Self {
        Self {
            dfa,
            map,
            _marker: PhantomData,
        }
    }
}

impl<A, S, F> DeterministicFiniteAutoMaton for SymbolMapDFA<A, S, F>
where
    A: DeterministicFiniteAutoMaton,
    F: Fn(&S) -> A::Symbol,
{
    type Symbol = S;
    type State = A::State;

    fn initial_state(&self) -> Self::State {
        self.dfa.initial_state()
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        self.dfa.transition(state, &(self.map)(c))
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.dfa.is_accepting(state)
    }

    fn state_to_index(&self, state: &Self::State) -> usize {
        self.dfa.state_to_index(state)
    }

    fn state_num(&self) -> usize {
        self.dfa.state_num()
    }
}
