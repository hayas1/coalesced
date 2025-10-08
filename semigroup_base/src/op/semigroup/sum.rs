use std::ops::Add;

use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = SumExt, commutative)]
pub struct Sum<T: Add<Output = T>>(pub T);
impl<T: Add<Output = T>> Semigroup for Sum<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(base.0 + other.0)
    }
}
#[cfg(feature = "monoid")]
impl<T: Add<Output = T> + num::Zero> crate::monoid::Monoid for Sum<T> {
    fn unit() -> Self {
        Self(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_sum_as_semigroup_op() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_sum_as_monoid() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_sum_commutative() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_sum() {
        let (a, b) = (Sum(1), Sum(2));
        assert_eq!(a.sum(b).into_inner(), 3);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.sum(rb).0.into_inner(), 3);
    }
}
