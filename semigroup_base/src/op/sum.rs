use std::ops::Add;

use semigroup_derive::{properties, ConstructionUse};

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

/// A semigroup construction that returns the sum.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup_base::{semigroup::Semigroup, op::{Construction, sum::Sum}};
///
/// let a = Sum(1);
/// let b = Sum(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
#[properties(monoid, commutative)]
pub struct Sum<T: Add<Output = T>>(pub T);
impl<T: Add<Output = T>> Semigroup for Sum<T> {
    fn op(base: Self, other: Self) -> Self {
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
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup};

    use super::*;

    #[test]
    fn test_sum_as_semigroup_op() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_semigroup!(a, b, c);
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
        assert_eq!(a.semigroup(b).into_inner(), 3);
        assert_eq!(b.semigroup(a).into_inner(), 3);
    }
}
