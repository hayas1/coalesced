use std::ops::Mul;

use semigroup_derive::{properties, ConstructionPriv};

use crate::Semigroup;

/// A semigroup construction that returns the product.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::prod::Prod, Construction, Semigroup};
///
/// let a = Prod(1);
/// let b = Prod(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative)]
#[properties(monoid, commutative)]
pub struct Prod<T: Mul<Output = T>>(pub T);
impl<T: Mul<Output = T>> Semigroup for Prod<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(base.0 * other.0)
    }
}
#[cfg(feature = "monoid")]
impl<T: Mul<Output = T> + num::One> Default for Prod<T> {
    fn default() -> Self {
        Self(T::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_prod_as_semigroup() {
        let (a, b, c) = (Prod(1), Prod(2), Prod(3));
        assert_semigroup!(a, b, c);
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
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }
}
