use hdrhistogram::{Counter, Histogram};
use semigroup_derive::ConstructionUse;

use crate::{commutative::Commutative, op::Construction, semigroup::Semigroup};

pub const DEFAULT_SIGFIG: u8 = 3;

#[derive(Debug, Clone, PartialEq, ConstructionUse)]
#[construction(commutative)]
pub struct HdrHistogram<T: Counter>(pub Histogram<T>);
impl<T: Counter> Semigroup for HdrHistogram<T> {
    fn semigroup_op(mut base: Self, other: Self) -> Self {
        base.0 += other.0;
        base
    }
}
#[cfg(feature = "monoid")]
impl<T: Counter> crate::monoid::Monoid for HdrHistogram<T> {
    fn unit() -> Self {
        Self(Histogram::new(DEFAULT_SIGFIG).unwrap_or_else(|_| unreachable!()))
    }
}
impl<T: Counter> From<u64> for HdrHistogram<T> {
    fn from(value: u64) -> Self {
        let mut h = Histogram::new(DEFAULT_SIGFIG).unwrap_or_else(|_| unreachable!());
        h += value;
        Self(h)
    }
}
impl<T: Counter> FromIterator<u64> for HdrHistogram<T> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut h = Histogram::new(DEFAULT_SIGFIG).unwrap_or_else(|_| unreachable!());
        for v in iter {
            h += v;
        }
        Self(h)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_hdr_histogram_as_semigroup_op() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_hdr_histogram_as_monoid() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_hdr_histogram_commutative() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        assert_commutative!(a, b, c);
    }

    #[test]
    fn test_hdr_histogram() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();

        let res = a.clone().semigroup(b.clone());
        assert_eq!(res.max(), 6);
        assert_eq!(res.min(), 1);
        assert_eq!(res.mean(), 3.5);
        assert_eq!(res.len(), 6);
        assert_eq!(res.value_at_quantile(0.5), 3);
        assert_eq!(res.value_at_quantile(0.9), 6);

        let res = b.semigroup(a);
        assert_eq!(res.max(), 6);
        assert_eq!(res.min(), 1);
        assert_eq!(res.mean(), 3.5);
        assert_eq!(res.len(), 6);
        assert_eq!(res.value_at_quantile(0.5), 3);
        assert_eq!(res.value_at_quantile(0.9), 6);
    }
}
