use num::{Integer, Unsigned};
use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = LcmExt, commutative)]
pub struct Lcm<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> Semigroup for Lcm<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(num::integer::lcm(base.0, other.0))
    }
}
impl<T: Unsigned + Integer + Clone> crate::monoid::Monoid for Lcm<T> {
    fn unit() -> Self {
        Self(T::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_lcm_as_semigroup_op() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_lcm_as_monoid() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_lcm_commutative() {
        let (a, b, c) = (Lcm(4u32), Lcm(6), Lcm(9));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_lcm() {
        let (a, b) = (Lcm(12u32), Lcm(18));
        assert_eq!(a.lcm(b).into_inner(), 36);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.lcm(rb).0.into_inner(), 36);
    }
}
