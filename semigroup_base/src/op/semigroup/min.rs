use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
pub struct Min<T: Ord>(pub T);
impl<T: Ord> Semigroup for Min<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(std::cmp::min(base.0, other.0))
    }
}
#[cfg(feature = "monoid")]
impl<T: Ord + num::Bounded> crate::monoid::Monoid for Min<T> {
    fn unit() -> Self {
        Self(T::max_value())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_commutative, assert_monoid, commutative::Reverse, semigroup::tests::assert_semigroup_op,
    };

    use super::*;

    #[test]
    fn test_min_as_semigroup_op() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_min_as_monoid() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_min_commutative() {
        let (a, b, c) = (Min(1), Min(2), Min(3));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_min() {
        let (a, b) = (Min(1), Min(2));
        assert_eq!(a.semigroup(b).into_inner(), 1);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.semigroup(rb).0.into_inner(), 1);
    }
}
