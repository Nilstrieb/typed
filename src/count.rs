use crate::helper::{Nat, S, Z};

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
