use num::{Integer, Unsigned};
use semigroup_derive::{properties, ConstructionUse};

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

/// A semigroup construction that returns the greatest common divisor.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup_base::{semigroup::Semigroup, op::{Construction, gcd::Gcd}};
///
/// let a = Gcd(12u32);
/// let b = Gcd(18);
///
/// assert_eq!(a.semigroup(b).into_inner(), 6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
#[properties(monoid, commutative)]
pub struct Gcd<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> Semigroup for Gcd<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(num::integer::gcd(base.0, other.0))
    }
}
impl<T: Unsigned + Integer + Clone> crate::monoid::Monoid for Gcd<T> {
    fn unit() -> Self {
        Self(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup};

    use super::*;

    #[test]
    fn test_gcd_as_semigroup_op() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_gcd_as_monoid() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_gcd_commutative() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_gcd() {
        let (a, b) = (Gcd(57u32), Gcd(95));
        assert_eq!(a.semigroup(b).into_inner(), 19);
        assert_eq!(b.semigroup(a).into_inner(), 19);
    }
}
