use std::ops::Mul;

use semigroup_derive::{properties_priv, ConstructionPriv};

use crate::Semigroup;

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) that returns the product.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Prod, Construction, Semigroup};
///
/// let a = Prod(1);
/// let b = Prod(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative, unit = Self(T::one()), unit_where = "T: num::One")]
#[properties_priv(monoid, commutative, unit_where = "T: num::One")]
pub struct Prod<T: Mul<Output = T>>(pub T);
impl<T: Mul<Output = T>> Semigroup for Prod<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(base.0 * other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_prod_semigroup() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_prod_monoid() {
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
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }
}
