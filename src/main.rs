#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use crate::helper::Bound;
use crate::index::SafeIdx;

mod count;
mod helper;
mod index;

fn main() {
    count::count();

    let array = [1, 2, 3];
    let bound_1 = Bound::<3>::new(2).unwrap();
    let bound_2 = Bound::<1>::new(0).unwrap();

    println!("{}", array.safe_idx(bound_1));
    println!("{}", array.safe_idx(bound_2));
}
