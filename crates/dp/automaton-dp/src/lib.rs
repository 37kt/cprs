use std::{cmp::Ordering, collections::HashMap, marker::PhantomData};

pub trait DeterministicFiniteAutoMaton {
    type Symbol;
    type State;

    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State>;
    fn is_accepting(&self, state: &Self::State) -> bool;
}

#[derive(Clone, Copy)]
pub struct Intersection<A0, A1>(pub A0, pub A1)
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>;

impl<A0, A1> DeterministicFiniteAutoMaton for Intersection<A0, A1>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
{
    type Symbol = A0::Symbol;
    type State = (A0::State, A1::State);

    fn initial_state(&self) -> Self::State {
        (self.0.initial_state(), self.1.initial_state())
    }

    fn transition(&self, state: &Self::State, c: &Self::Symbol) -> Option<Self::State> {
        let (st0, st1) = state;
        let st0 = self.0.transition(st0, c)?;
        let st1 = self.1.transition(st1, c)?;
        Some((st0, st1))
    }

    fn is_accepting(&self, state: &Self::State) -> bool {
        let (st0, st1) = state;
        self.0.is_accepting(st0) && self.1.is_accepting(st1)
    }
}

impl<A0, A1> Intersection<A0, A1>
where
    A0: DeterministicFiniteAutoMaton,
    A1: DeterministicFiniteAutoMaton<Symbol = A0::Symbol>,
{
    pub fn new(a0: A0, a1: A1) -> Self {
        Self(a0, a1)
    }
}

#[derive(Clone, Copy)]
pub struct LexicographicalDfa<'a, T> {
    seq: &'a [T],
    ord: Ordering,
    eq: bool,
}

impl<'a, T> DeterministicFiniteAutoMaton for LexicographicalDfa<'a, T>
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
}

impl<'a, T> LexicographicalDfa<'a, T>
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
pub struct ReversedLexicographicalDfa<'a, T> {
    seq: &'a [T],
    ord: Ordering,
    eq: bool,
}

impl<'a, T> DeterministicFiniteAutoMaton for ReversedLexicographicalDfa<'a, T>
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
}

impl<'a, T> ReversedLexicographicalDfa<'a, T>
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
pub struct SymbolMap<A, S, F>
where
    A: DeterministicFiniteAutoMaton,
    F: Fn(&S) -> A::Symbol,
{
    dfa: A,
    map: F,
    _marker: PhantomData<fn() -> S>,
}

impl<A, S, F> SymbolMap<A, S, F>
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

impl<A, S, F> DeterministicFiniteAutoMaton for SymbolMap<A, S, F>
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
}

pub fn automaton_dp<A, T>(
    dfa: A,
    alphabet: impl Iterator<Item = A::Symbol> + Clone,
    len: usize,
    mut act: impl FnMut(&A::Symbol, &T) -> T,
    mut op: impl FnMut(&T, &T) -> T,
    e: T,
    dp_initial: T,
) -> T
where
    A: DeterministicFiniteAutoMaton,
    A::State: Eq + std::hash::Hash,
    T: Clone,
{
    let mut dp = HashMap::<A::State, T>::new();
    let mut ndp = HashMap::<A::State, T>::new();
    dp.insert(dfa.initial_state(), dp_initial);

    for _ in 0..len {
        for (st, val) in dp.drain() {
            for c in alphabet.clone() {
                if let Some(nst) = dfa.transition(&st, &c) {
                    let nval = act(&c, &val);
                    ndp.entry(nst)
                        .and_modify(|sum| *sum = op(sum, &nval))
                        .or_insert(nval);
                }
            }
        }
        std::mem::swap(&mut dp, &mut ndp);
    }

    dp.into_iter()
        .filter(|(st, _)| dfa.is_accepting(st))
        .fold(e, |acc, (_, val)| op(&acc, &val))
}
