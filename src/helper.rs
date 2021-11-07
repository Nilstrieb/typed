use std::marker::PhantomData;

/////////// type level arithmetic

pub trait Nat {
    fn int() -> i32;
}

pub struct Z;

impl Nat for Z {
    fn int() -> i32 {
        0
    }
}

pub struct S<N: Nat>(PhantomData<N>);

impl<N: Nat> Nat for S<N> {
    fn int() -> i32 {
        N::int() + 1
    }
}

/////////// bound numbers

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bound<const UPPER: usize>(usize);

impl<const UPPER: usize> Bound<UPPER> {
    pub fn new(num: usize) -> Option<Bound<UPPER>> {
        if num < UPPER {
            Some(Self(num))
        } else {
            None
        }
    }
}

impl<const UPPER: usize> Bound<UPPER> {
    pub fn inner(&self) -> usize {
        self.0
    }
}

//////////// boolean expression testing

pub trait True {}

pub trait False {}

pub struct Test<const COND: bool>();

impl True for Test<true> {}
impl False for Test<false> {}

pub struct Not<B>(PhantomData<B>);

impl<B> True for Not<B> where B: False {}
impl<B> False for Not<B> where B: True {}
