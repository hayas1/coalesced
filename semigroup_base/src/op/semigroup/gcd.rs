use num::{integer::gcd, Integer, Unsigned};
use semigroup_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = GcdExt)]
pub struct Gcd<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> Semigroup for Gcd<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(gcd(base.0, other.0))
    }
}
impl<T: Unsigned + Integer + Clone> crate::monoid::Monoid for Gcd<T> {
    fn unit() -> Self {
        Self(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_gcd_as_semigroup_op() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_gcd_as_monoid() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_gcd() {
        let (a, b) = (Gcd(57u32), Gcd(95));
        assert_eq!(a.gcd(b).into_inner(), 19);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.gcd(rb).0.into_inner(), 19);
    }
}
