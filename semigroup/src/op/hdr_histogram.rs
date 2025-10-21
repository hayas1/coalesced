use std::borrow::Cow;

use hdrhistogram::{Counter, Histogram};
use semigroup_derive::{properties_priv, ConstructionPriv};

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
#[derive(Debug, Clone, PartialEq, ConstructionPriv)]
#[construction(monoid, commutative, unit = Self(HdrHistogramInner::new()), without_from_impl)]
#[properties_priv(monoid, commutative)]
pub struct HdrHistogram<T: Counter>(pub HdrHistogramInner<T>);
impl<T: Counter> Semigroup for HdrHistogram<T> {
    fn op(base: Self, other: Self) -> Self {
        Self(HdrHistogramInner::op(base.0, other.0))
    }
}
impl<T: Counter, U: Into<HdrHistogramInner<T>>> From<U> for HdrHistogram<T> {
    fn from(value: U) -> Self {
        Self(value.into())
    }
}
impl<T: Counter> FromIterator<u64> for HdrHistogram<T> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        Self(HdrHistogramInner::from_iter(iter))
    }
}
impl<T: Counter> HdrHistogram<T> {
    pub fn histogram(&self) -> Cow<Histogram<T>> {
        self.0.histogram()
    }
    pub fn into_histogram(self) -> Histogram<T> {
        self.0.into_histogram()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HdrHistogramInner<T: Counter> {
    Value(u64),
    Histogram(Histogram<T>),
}
impl<T: Counter> Semigroup for HdrHistogramInner<T> {
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
impl<T: Counter> From<u64> for HdrHistogramInner<T> {
    fn from(value: u64) -> Self {
        Self::Value(value)
    }
}
impl<T: Counter> From<Histogram<T>> for HdrHistogramInner<T> {
    fn from(value: Histogram<T>) -> Self {
        Self::Histogram(value)
    }
}
impl<T: Counter> FromIterator<u64> for HdrHistogramInner<T> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut h = Self::base_histogram();
        for v in iter {
            h += v;
        }
        h.into()
    }
}
impl<T: Counter> From<HdrHistogramInner<T>> for Histogram<T> {
    fn from(value: HdrHistogramInner<T>) -> Self {
        match value {
            HdrHistogramInner::Value(v) => HdrHistogramInner::value_histogram(v),
            HdrHistogramInner::Histogram(h) => h,
        }
    }
}
impl<T: Counter> HdrHistogramInner<T> {
    fn new() -> Self {
        Self::Histogram(Self::base_histogram())
    }
    fn base_histogram() -> Histogram<T> {
        Histogram::new(3).unwrap_or_else(|_| unreachable!())
    }
    fn value_histogram(value: u64) -> Histogram<T> {
        Some(value).into_iter().collect::<Self>().into()
    }
    fn histogram(&self) -> Cow<Histogram<T>> {
        match self {
            Self::Value(v) => Cow::Owned(Self::value_histogram(*v)),
            Self::Histogram(h) => Cow::Borrowed(h),
        }
    }
    fn into_histogram(self) -> Histogram<T> {
        self.into()
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
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();

        let histogram = a.clone().semigroup(b.clone()).into_histogram();
        assert_eq!(histogram.max(), 6);
        assert_eq!(histogram.min(), 1);
        assert_eq!(histogram.mean(), 3.5);
        assert_eq!(histogram.len(), 6);
        assert_eq!(histogram.value_at_quantile(0.5), 3);
        assert_eq!(histogram.value_at_quantile(0.9), 6);

        let histogram = b.semigroup(a).into_histogram();
        assert_eq!(histogram.max(), 6);
        assert_eq!(histogram.min(), 1);
        assert_eq!(histogram.mean(), 3.5);
        assert_eq!(histogram.len(), 6);
        assert_eq!(histogram.value_at_quantile(0.5), 3);
        assert_eq!(histogram.value_at_quantile(0.9), 6);
    }
}
