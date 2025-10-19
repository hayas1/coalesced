use std::ops::BitXor;

use semigroup_derive::{properties, ConstructionPriv};

use crate::Semigroup;

/// A semigroup construction that returns the exclusive or.
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::Xor, Construction, Semigroup};
///
/// let a = Xor(0b101);
/// let b = Xor(0b100);
///
/// assert_eq!(a.semigroup(b).into_inner(), 0b001);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionPriv)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(monoid, commutative, unit = Self(T::zero()), unit_where = "T: num::Zero")]
#[properties(monoid, commutative)]
pub struct Xor<T: BitXor<Output = T>>(pub T);
impl<T: BitXor<Output = T>> Semigroup for Xor<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(base.0 ^ other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Construction, Semigroup};

    use super::*;

    #[test]
    fn test_xor_as_semigroup() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        assert_semigroup!(a, b, c);
    }

    #[test]
    fn test_xor_as_monoid() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_xor_commutative() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_xor() {
        let (a, b) = (Xor(0b101), Xor(0b100));
        assert_eq!(a.semigroup(b).into_inner(), 0b001);
        assert_eq!(b.semigroup(a).into_inner(), 0b001);
    }
}
