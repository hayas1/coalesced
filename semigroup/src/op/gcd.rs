use num::{Integer, Unsigned};
use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::Semigroup;

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the greatest common divisor.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Gcd, Construction, Semigroup};
///
/// let a = Gcd(12u32);
/// let b = Gcd(18);
///
/// assert_eq!(a.semigroup(b).into_inner(), 6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative, identity = Self(T::zero()))]
#[properties_priv(monoid, commutative)]
pub struct Gcd<T: Unsigned + Integer + Clone>(pub T);
impl<T: Unsigned + Integer + Clone> Semigroup for Gcd<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(num::integer::gcd(base.0, other.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_gcd_semigroup() {
        let (a, b, c) = (Gcd(12u32), Gcd(18), Gcd(27));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_gcd_monoid() {
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
