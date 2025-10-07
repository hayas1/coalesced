use crate::semigroup::Semigroup;

pub trait Commutative: Semigroup {}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use super::*;

    #[macro_export]
    macro_rules! assert_commutative {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::commutative::tests::assert_commutative!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::semigroup::tests::pick3($v);
                $crate::commutative::tests::assert_commutative_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }
    pub use assert_commutative;

    pub fn assert_commutative_impl<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone());
        let bca = T::semigroup_op(T::semigroup_op(b.clone(), c.clone()), a.clone());
        let cba = T::semigroup_op(T::semigroup_op(c.clone(), b.clone()), a.clone());
        let acb = T::semigroup_op(T::semigroup_op(a.clone(), c.clone()), b.clone());
        let bac = T::semigroup_op(T::semigroup_op(b.clone(), a.clone()), c.clone());
        let cab = T::semigroup_op(T::semigroup_op(c.clone(), a.clone()), b.clone());

        assert_eq!(abc, bca);
        assert_eq!(bca, cba);
        assert_eq!(cba, acb);
        assert_eq!(acb, bac);
        assert_eq!(bac, cab);
        assert_eq!(cab, abc);
    }
}
