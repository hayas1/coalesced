use hdrhistogram::{Counter, Histogram};
use semigroup_derive::ConstructionUse;

use crate::{op::Construction, reverse::Reverse, semigroup::Semigroup};

#[derive(Debug, Clone, PartialEq, ConstructionUse)]
#[construction(op_trait = HdrHistogramExt)]
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
        Self(Histogram::new(3).unwrap_or_else(|_| unreachable!()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_monoid, semigroup::tests::assert_semigroup_op};

    use super::*;

    #[test]
    fn test_hdr_histogram_as_semigroup_op() {
        let mut h1 = Histogram::<u32>::new(3).unwrap();
        h1 += 1;
        let mut h2 = Histogram::new(3).unwrap();
        h2 += 2;
        let mut h3 = Histogram::new(3).unwrap();
        h3 += 3;
        let (a, b, c) = (HdrHistogram(h1), HdrHistogram(h2), HdrHistogram(h3));
        assert_semigroup_op!(a, b, c);
    }

    #[test]
    fn test_hdr_histogram_as_monoid() {
        let mut h1 = Histogram::<u32>::new(3).unwrap();
        h1 += 1;
        let mut h2 = Histogram::new(3).unwrap();
        h2 += 2;
        let mut h3 = Histogram::new(3).unwrap();
        h3 += 3;
        let (a, b, c) = (HdrHistogram(h1), HdrHistogram(h2), HdrHistogram(h3));
        assert_monoid!(a, b, c);
    }

    #[test]
    fn test_hdr_histogram() {
        let mut h1 = Histogram::<u32>::new(3).unwrap();
        h1 += 1;
        let mut h2 = Histogram::new(3).unwrap();
        h2 += 2;

        let (a, b) = (HdrHistogram(h1), HdrHistogram(h2));
        let res = a.clone().hdr_histogram(b.clone());
        assert_eq!(res.max(), 2);
        assert_eq!(res.min(), 1);
        assert_eq!(res.mean(), 1.5);
        assert_eq!(res.len(), 2);
        assert_eq!(res.value_at_quantile(0.5), 1);
        assert_eq!(res.value_at_quantile(0.9), 2);

        let (ra, rb) = (Reverse(a), Reverse(b));
        let Reverse(res) = ra.hdr_histogram(rb);
        assert_eq!(res.max(), 2);
        assert_eq!(res.min(), 1);
        assert_eq!(res.mean(), 1.5);
        assert_eq!(res.len(), 2);
        assert_eq!(res.value_at_quantile(0.5), 1);
        assert_eq!(res.value_at_quantile(0.9), 2);
    }
}
