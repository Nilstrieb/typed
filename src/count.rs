use std::marker::PhantomData;

trait Nat {
    fn int() -> i32;
}

struct Z;

impl Nat for Z {
    fn int() -> i32 {
        0
    }
}

struct S<N: Nat>(PhantomData<N>);

impl<N: Nat> Nat for S<N> {
    fn int() -> i32 {
        N::int() + 1
    }
}


trait Count: Nat {
    fn count() -> String;
}

impl Count for Z {
    fn count() -> String {
        "0".to_string()
    }
}

impl<N: Count> Count for S<N> {
    fn count() -> String {
        format!("{} {}", N::count(), Self::int())
    }
}

pub fn count() {
    println!("{}", <S<S<S<Z>>> as Count>::count());
}