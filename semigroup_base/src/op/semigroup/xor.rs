use std::ops::BitXor;

use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, ConstructionUse)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[construction(op_trait = XorExt, commutative)]
pub struct Xor<T: BitXor<Output = T>>(pub T);
impl<T: BitXor<Output = T>> Semigroup for Xor<T> {
    fn semigroup_op(base: Self, other: Self) -> Self {
        Self(base.0 ^ other.0)
    }
}
#[cfg(feature = "monoid")]
impl<T: BitXor<Output = T> + num::Zero> crate::monoid::Monoid for Xor<T> {
    fn unit() -> Self {
        Self(T::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_xor_as_semigroup_op() {
        let (a, b, c) = (Xor(0b111), Xor(0b101), Xor(0b100));
        assert_semigroup_op!(a, b, c);
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
        assert_eq!(a.xor(b).into_inner(), 0b001);

        let (ra, rb) = (Reverse(a), Reverse(b));
        assert_eq!(ra.xor(rb).0.into_inner(), 0b001);
    }
}
