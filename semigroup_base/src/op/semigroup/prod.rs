use std::ops::Mul;

use semigroup_derive::{properties, ConstructionUse};

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

/// A semigroup construction that returns the product.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup_base::{semigroup::Semigroup, op::{Construction, semigroup::prod::Prod}};
///
/// let a = Prod(1);
/// let b = Prod(2);
///
/// assert_eq!(a.semigroup(b).into_inner(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(commutative)]
#[properties(monoid, commutative)]
pub struct Prod<T: Mul<Output = T>>(pub T);
impl<T: Mul<Output = T>> Semigroup for Prod<T> {
    fn op(base: Self, other: Self) -> Self {
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
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup_op};

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
        assert_eq!(b.semigroup(a).into_inner(), 2);
    }
}
