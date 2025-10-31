use std::borrow::Cow;

use hdrhistogram::{Counter, Histogram};
use semigroup_derive::{properties_priv, ConstructionPriv, SemigroupPriv};

use crate::Semigroup;

/// A [`Semigroup`](crate::Semigroup) [construction](crate::Construction) merging two `HdrHistogram`s.
///
/// Histogram can be used to aggregate data with the following properties:
/// - mean
/// - quantile
/// - and more...
///
/// # Properties
/// <!-- properties -->
///
/// # Examples
/// ## Statistically aggregation
/// ```
/// use semigroup::{op::HdrHistogram, Semigroup};
///
/// let histogram1 = (1..100).collect::<HdrHistogram<u32>>();
/// let histogram2 = (100..1000).collect::<HdrHistogram<u32>>();
///
/// let histogram = histogram1.semigroup(histogram2).into_histogram();
///
/// assert_eq!(histogram.mean(), 499.9999999999999);
/// assert_eq!(histogram.value_at_quantile(0.9), 900);
/// ```
///
/// ## Load testing request-response result
/// ```
/// # #[cfg(feature="monoid")]
/// # {
/// use std::time::{Duration, Instant};
/// use semigroup::{
///     op::{HdrHistogram, Sum, Min, Max},
///     Construction, Commutative, Semigroup, OptionMonoid, Monoid
/// };
///
/// #[derive(Debug, Clone, PartialEq, Semigroup)]
/// #[semigroup(commutative)]
/// pub struct RequestAggregate {
///     count: Sum<u64>,
///     pass: Sum<u64>,
///     start: Min<Instant>,
///     end: Max<Instant>,
///     latency: HdrHistogram<u32>,
/// }
/// impl RequestAggregate {
///     pub fn new(pass: bool, time: Instant, latency: Duration) -> Self {
///         Self {
///             count: Sum(1),
///             pass: Sum(if pass { 1 } else { 0 }),
///             start: Min(time),
///             end: Max(time),
///             latency: HdrHistogram::from(latency.as_millis() as u64),
///         }
///     }
///     pub fn count(&self) -> u64 {
///         self.count.into_inner()
///     }
///     pub fn pass_rate(&self) -> f64 {
///         self.pass.into_inner() as f64 / self.count.into_inner() as f64
///     }
///     pub fn duration(&self) -> Duration {
///         self.end.into_inner() - self.start.into_inner()
///     }
///     pub fn rps(&self) -> f64 {
///         self.count.into_inner() as f64 / self.duration().as_secs_f64()
///     }
///     pub fn p99_latency(&self) -> Duration {
///         Duration::from_millis(self.latency.histogram().value_at_quantile(0.99) as u64)
///     }
/// }
///
/// let (now, mut agg) = (Instant::now(), OptionMonoid::identity());
/// for i in 0..10000000 {
///     let duration = Duration::from_millis(i);
///     agg = agg.semigroup(RequestAggregate::new(i % 2 == 0, now + duration, duration).into());
/// }
///
/// let request_aggregate = agg.into_inner().unwrap();
/// assert_eq!(request_aggregate.count(), 10000000);
/// assert_eq!(request_aggregate.pass_rate(), 0.5);
/// assert_eq!(request_aggregate.duration(), Duration::from_millis(9999999));
/// assert_eq!(request_aggregate.rps(), 1000.00010000001);
/// assert_eq!(request_aggregate.p99_latency(), Duration::from_millis(9904127));
/// # }
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
#[construction(monoid, commutative, identity = HdrHistogramInner::new(), without_construction)]
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
    use crate::Semigroup;

    use super::*;

    #[test]
    fn test_hdr_histogram_semigroup() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        crate::assert_semigroup!(a, b, c);
    }

    #[test]
    #[cfg(feature = "monoid")]
    fn test_hdr_histogram_monoid() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        crate::assert_monoid!(a, b, c);
    }

    #[test]
    #[cfg(feature = "commutative")]
    fn test_hdr_histogram_commutative() {
        let a: HdrHistogram<u32> = [1u64, 2, 3].into_iter().collect();
        let b: HdrHistogram<u32> = [4, 5, 6].into_iter().collect();
        let c: HdrHistogram<u32> = [7, 8, 9].into_iter().collect();
        crate::assert_commutative!(a, b, c);
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
