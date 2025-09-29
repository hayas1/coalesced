use crate::annotate::Annotated;

pub trait Semigroup {
    fn semigroup_op(base: Self, other: Self) -> Self;
    fn semigroup(self, other: Self) -> Self
    where
        Self: Sized,
    {
        Semigroup::semigroup_op(self, other)
    }
}
pub trait AnnotatedSemigroup<A>: Sized + Semigroup {
    fn annotated_op(base: Annotated<Self, A>, other: Annotated<Self, A>) -> Annotated<Self, A>;
}

#[cfg(any(test, feature = "test"))]
pub mod tests {
    use std::fmt::Debug;

    use crate::{
        iter::tests::assert_lazy_evaluation_iter,
        reverse::tests::{assert_reverse, assert_reverse_associative_law},
    };

    use super::*;

    #[macro_export]
    macro_rules! assert_semigroup_op {
        ($a:expr, $b: expr, $c: expr) => {
            $crate::semigroup::tests::assert_semigroup_op_impl($a.clone(), $b.clone(), $c.clone());
            $crate::monoid::tests::assert_option_monoid($a.clone(), $b.clone(), $c.clone());
        };
    }
    pub use assert_semigroup_op;

    pub fn assert_semigroup_op_impl<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        assert_associative_law(a.clone(), b.clone(), c.clone());
        assert_reverse(a.clone(), b.clone(), c.clone());
        assert_reverse_associative_law(a.clone(), b.clone(), c.clone());
        assert_lazy_evaluation_iter(a.clone(), b.clone(), c.clone());
    }

    pub fn assert_associative_law<T: Semigroup + Clone + PartialEq + Debug>(a: T, b: T, c: T) {
        let ab_c = T::semigroup_op(T::semigroup_op(a.clone(), b.clone()), c.clone());
        let a_bc = T::semigroup_op(a.clone(), T::semigroup_op(b.clone(), c.clone()));
        assert_eq!(ab_c, a_bc);
    }
}
