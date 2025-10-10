use crate::semigroup::Semigroup;

pub trait Commutative: Semigroup {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(Semigroup::semigroup_op(other.0, base.0))
    }
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use crate::semigroup::tests::assert_associative_law;

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

    pub fn assert_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            T::semigroup_op(a.clone(), b.clone()),
            Reverse::<T>::semigroup_op(rb.clone(), ra.clone()).0
        );
        assert_eq!(
            T::semigroup_op(b.clone(), c.clone()),
            Reverse::<T>::semigroup_op(rc.clone(), rb.clone()).0
        );
        assert_eq!(
            T::semigroup_op(a.clone(), c.clone()),
            Reverse::<T>::semigroup_op(rc.clone(), ra.clone()).0
        );
    }
    pub fn assert_reverse_associative_law<T: Semigroup + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a), Reverse(b), Reverse(c));
        assert_associative_law(ra, rb, rc);
    }
}
