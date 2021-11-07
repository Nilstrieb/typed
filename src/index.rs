use crate::helper::{Bound, Test, True};

pub trait SafeIdx<const SIZE: usize> {
    type Output;

    fn safe_idx(&self, num: Bound<SIZE>) -> &Self::Output;
}

impl<const BOUND: usize, const SIZE: usize, T> SafeIdx<BOUND> for [T; SIZE]
where
    Test<{ BOUND <= SIZE }>: True,
{
    type Output = T;

    fn safe_idx(&self, num: Bound<BOUND>) -> &Self::Output {
        // SAFETY: We can be sure that we are not out of bounds because
        // `num` will always be less than `BOUND`, and `BOUND` will always
        // be less than or equal to our SIZE
        unsafe { self.get_unchecked(num.inner()) }
    }
}
