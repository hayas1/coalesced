use std::ops::Mul;

use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
pub struct Prod<T: Mul<Output = T>>(pub T);
impl<T: Mul<Output = T>> Semigroup for Prod<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(base.0 * other.0)
    }
}
#[cfg(feature = "monoid")]
impl<T: Mul<Output = T> + num::One> crate::monoid::Monoid for Prod<T> {
    fn unit() -> Self {
        Self(T::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_commutative, assert_monoid, reverse::Reverse, semigroup::tests::assert_semigroup_op,
    };

    use super::*;

    #[test]
    fn test_prod_as_semigroup_op() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_prod_as_monoid() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_prod_commutative() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_prod() {
        let (a, b) = (Prod(1), Prod(2));
        assert_eq!(a.semigroup(b).into_inner(), 2);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.semigroup(rb).0.into_inner(), 2);
    }
}
