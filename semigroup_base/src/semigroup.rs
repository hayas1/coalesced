use crate::annotate::Annotated;

pub trait Semigroup {
    fn op(base: Self, other: Self) -> Self;
    fn semigroup(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Semigroup::op(self, other)
    }
}
pub trait AnnotatedSemigroup<A>: Sized + Semigroup {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use rand::seq::IndexedRandom;

    use crate::{
        commutative::tests::{assert_reverse_associative_law, assert_reverse_reverse},
        iter::tests::assert_lazy_evaluation_iter,
    };

    use super::*;

    #[macro_export]
    macro_rules! assert_semigroup {
        ($a:expr, $b: expr, $($tail: expr),*) => {
            {
                let v = vec![$a, $b, $($tail),*];
                $crate::semigroup::tests::assert_semigroup!(&v)
            }
        };
        ($v:expr) => {
            {
                let (a, b, c) = $crate::semigroup::tests::pick3($v);
                $crate::semigroup::tests::assert_semigroup_impl(a.clone(), b.clone(), c.clone());
                $crate::monoid::tests::assert_option_monoid(a.clone(), b.clone(), c.clone());
            }
        };
    }
    pub use assert_semigroup;

    pub fn pick3<T: Clone>(data: &[T]) -> (T, T, T) {
        data.choose_multiple_array(&mut rand::rng())
            .map(|[a, b, c]| (a, b, c))
            .expect("failed to pick 3 items")
    }

    pub fn assert_semigroup_impl<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reverse_reverse(a.clone(), b.clone(), c.clone());
        assert_reverse_associative_law(a.clone(), b.clone(), c.clone());
        assert_lazy_evaluation_iter(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = T::op(T::op(a.clone(), b.clone()), c.clone());
        let a_bc = T::op(a.clone(), T::op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }
}
