use num::{Integer, Unsigned};
use semigroup_derive::{properties, ConstructionUse};

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

/// A semigroup construction that returns the least common multiple.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup_base::{semigroup::Semigroup, op::{Construction, semigroup::lcm::Lcm}};
///
/// let a = Lcm(12u32);
/// let b = Lcm(18);
///
/// assert_eq!(a.semigroup(b).into_inner(), 36);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
#[properties(monoid, commutative)]
pub struct Lcm<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> Semigroup for Lcm<T> {
    fn op(base: Self, other: Self) -> Self {
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
        assert_eq!(a.semigroup(b).into_inner(), 36);
        assert_eq!(b.semigroup(a).into_inner(), 36);
    }
}
