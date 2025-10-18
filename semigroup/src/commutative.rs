use crate::Semigroup;

/// [`Commutative`] represents a binary operation that satisfies the following property
/// 1. *Commutativity*: `op(a, b) = op(b, a)`
///
/// The [*semigroup*](crate::Semigroup) set that satisfies the *commutativity* property is often called *commutative semigroup*.
///
/// And the [*monoid*](crate::Monoid) set that satisfies the *commutativity* property is often called *commutative monoid*.
///
/// # Deriving
/// [`Commutative`] can be derived like [`Semigroup`], use `commutative` attribute.
/// ```
/// use semigroup::Semigroup;
/// #[derive(Debug, Clone, PartialEq, Default, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct ExampleStruct {
///     #[semigroup(with = "semigroup::op::sum::Sum")]
///     pub sum: u32,
///     #[semigroup(with = "semigroup::op::min::Min")]
///     pub min: u32,
/// }
///
/// let a = ExampleStruct { sum: 1, min: 1 };
/// let b = ExampleStruct { sum: 10, min: 10 };
/// let c = ExampleStruct { sum: 100, min: 100 };
///
/// // #[test]
/// semigroup::assert_commutative!(&a, &b, &c);
/// assert_eq!(a.semigroup(b).semigroup(c), ExampleStruct { sum: 111, min: 1 });
/// ```
///
/// # Testing
/// Use [`crate::assert_commutative!`] macro.
/// This is marker trait.
///
/// The *commutativity* property is not guaranteed by Rust’s type system,
/// so it must be verified manually using [`crate::assert_commutative!`].
///
pub trait Commutative: Semigroup {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reverse<T>(pub T);

impl<T: Semigroup> Semigroup for Reverse<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(Semigroup::op(other.0, base.0))
    }
}

#[cfg(any(test, feature = "test"))]
pub mod test_commutative {
    use std::fmt::Debug;

    use crate::semigroup::test_semigroup::assert_associative_law;

    use super::*;

    #[macro_export]
    macro_rules! assert_commutative {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::assert_commutative!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::test_semigroup::pick3($v);
                $crate::test_commutative::assert_commutative_impl(a.clone(), b.clone(), c.clone());
            }
        };
    }

    pub fn assert_commutative_impl<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_commutative_law(a.clone(), b.clone(), c.clone());
        assert_commutative_reverse(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_commutative_law<T: Commutative + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let abc = T::op(T::op(a.clone(), b.clone()), c.clone());
        let bca = T::op(T::op(b.clone(), c.clone()), a.clone());
        let cba = T::op(T::op(c.clone(), b.clone()), a.clone());
        let acb = T::op(T::op(a.clone(), c.clone()), b.clone());
        let bac = T::op(T::op(b.clone(), a.clone()), c.clone());
        let cab = T::op(T::op(c.clone(), a.clone()), b.clone());

        assert_eq!(abc, bca);
        assert_eq!(bca, cba);
        assert_eq!(cba, acb);
        assert_eq!(acb, bac);
        assert_eq!(bac, cab);
        assert_eq!(cab, abc);
    }

    pub fn assert_commutative_reverse<T: Commutative + Clone + PartialEq + Debug>(
        a: T,
        b: T,
        c: T,
    ) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            T::op(a.clone(), b.clone()),
            Reverse::<T>::op(ra.clone(), rb.clone()).0
        );
        assert_eq!(
            T::op(b.clone(), c.clone()),
            Reverse::<T>::op(rb.clone(), rc.clone()).0
        );
        assert_eq!(
            T::op(c.clone(), a.clone()),
            Reverse::<T>::op(rc.clone(), ra.clone()).0
        );
    }

    pub fn assert_reverse_reverse<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let (ra, rb, rc) = (Reverse(a.clone()), Reverse(b.clone()), Reverse(c.clone()));
        assert_eq!(
            T::op(a.clone(), b.clone()),
            Reverse::<T>::op(rb.clone(), ra.clone()).0
        );
        assert_eq!(
            T::op(b.clone(), c.clone()),
            Reverse::<T>::op(rc.clone(), rb.clone()).0
        );
        assert_eq!(
            T::op(a.clone(), c.clone()),
            Reverse::<T>::op(rc.clone(), ra.clone()).0
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
