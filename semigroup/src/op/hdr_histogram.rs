use std::borrow::Cow;

use hdrhistogram::{Counter, Histogram};
use semigroup_derive::{properties_priv, ConstructionPriv, SemigroupPriv};

use crate::Semigroup;

/// A semigroup construction merging two `HdrHistogram`s.
/// - mean
/// - quantile
/// - and more...
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ```
/// use semigroup::{op::HdrHistogram, Construction, Semigroup};
///
/// let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
/// let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
///
/// let h = a.semigroup(b);
/// assert_eq!(h.histogram().mean(), 3.5);
/// assert_eq!(h.histogram().value_at_quantile(0.9), 6);
/// ```
#[derive(Debug, Clone, PartialEq, SemigroupPriv)]
#[semigroup(monoid, commutative)]
#[properties_priv(monoid, commutative)]
pub struct HdrHistogram<C: Counter>(HdrHistogramInner<C>);
impl<C: Counter, T: Into<HdrHistogramInner<C>>> From<T> for HdrHistogram<C> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
impl<C: Counter> FromIterator<u64> for HdrHistogram<C> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        Self(HdrHistogramInner::from_iter(iter))
    }
}
impl<C: Counter> From<HdrHistogram<C>> for Histogram<C> {
    fn from(value: HdrHistogram<C>) -> Self {
        value.0.into()
    }
}
impl<C: Counter> HdrHistogram<C> {
    pub fn histogram(&self) -> Cow<Histogram<C>> {
        self.0.histogram()
    }
    pub fn into_histogram(self) -> Histogram<C> {
        self.0.into()
    }
}

#[derive(Debug, Clone, PartialEq, ConstructionPriv)]
#[construction(monoid, commutative, unit = HdrHistogramInner::new(), without_construction)]
enum HdrHistogramInner<C: Counter> {
    Value(u64),
    Histogram(Histogram<C>),
}
impl<C: Counter> Semigroup for HdrHistogramInner<C> {
    fn op(base: Self, other: Self) -> Self {
        match (base, other) {
            (Self::Value(a), Self::Value(b)) => vec![a, b].into_iter().collect(),
            (Self::Value(a), Self::Histogram(mut b)) => {
                b += a;
                Self::Histogram(b)
            }
            (Self::Histogram(mut a), Self::Value(b)) => {
                a += b;
                Self::Histogram(a)
            }
            (Self::Histogram(mut a), Self::Histogram(b)) => {
                a += b;
                Self::Histogram(a)
            }
        }
    }
}
impl<C: Counter> From<u64> for HdrHistogramInner<C> {
    fn from(value: u64) -> Self {
        Self::Value(value)
    }
}
impl<C: Counter> From<Histogram<C>> for HdrHistogramInner<C> {
    fn from(value: Histogram<C>) -> Self {
        Self::Histogram(value)
    }
}
impl<C: Counter> FromIterator<u64> for HdrHistogramInner<C> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut h = Self::base_histogram();
        for v in iter {
            h += v;
        }
        h.into()
    }
}
impl<C: Counter> From<HdrHistogramInner<C>> for Histogram<C> {
    fn from(value: HdrHistogramInner<C>) -> Self {
        match value {
            HdrHistogramInner::Value(v) => HdrHistogramInner::value_histogram(v),
            HdrHistogramInner::Histogram(h) => h,
        }
    }
}
impl<C: Counter> HdrHistogramInner<C> {
    fn new() -> Self {
        Self::Histogram(Self::base_histogram())
    }
    fn base_histogram() -> Histogram<C> {
        Histogram::new(3).unwrap_or_else(|_| unreachable!())
    }
    fn value_histogram(value: u64) -> Histogram<C> {
        Some(value).into_iter().collect::<Self>().into()
    }
    fn histogram(&self) -> Cow<Histogram<C>> {
        match self {
            Self::Value(v) => Cow::Owned(Self::value_histogram(*v)),
            Self::Histogram(h) => Cow::Borrowed(h),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_commutative, assert_monoid, assert_semigroup, Semigroup};

    use super::*;

    #[test]
    fn test_hdr_histogram_as_semigroup() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        assert_semigroup!(a, b, c);
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
        let a = HdrHistogram::from(1);
        let b: HdrHistogram<u32> = [2, 3].into_iter().collect();
        let c: HdrHistogram<u32> = [4, 5].into_iter().collect();
        let d = HdrHistogram::from(6);

        let histogram = a
            .clone()
            .semigroup(b.clone())
            .semigroup(c.clone())
            .semigroup(d.clone())
            .into_histogram();
        assert_eq!(histogram.max(), 6);
        assert_eq!(histogram.min(), 1);
        assert_eq!(histogram.mean(), 3.5);
        assert_eq!(histogram.len(), 6);
        assert_eq!(histogram.value_at_quantile(0.5), 3);
        assert_eq!(histogram.value_at_quantile(0.9), 6);

        let histogram = a.semigroup(d).semigroup(b).semigroup(c).into_histogram();
        assert_eq!(histogram.max(), 6);
        assert_eq!(histogram.min(), 1);
        assert_eq!(histogram.mean(), 3.5);
        assert_eq!(histogram.len(), 6);
        assert_eq!(histogram.value_at_quantile(0.5), 3);
        assert_eq!(histogram.value_at_quantile(0.9), 6);
    }
}
