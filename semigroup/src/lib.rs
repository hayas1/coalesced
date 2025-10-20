#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! [*semigroup*](crate::semigroup::Semigroup) trait is useful for
//! - reading configs from multiple sources
//! - statistically aggregation
//! - fast range queries using segment tree
//!
//! # Usage
//! ```sh
//! cargo add semigroup --features derive,monoid
//! ```
//!
//! # Examples
//!
//! ## Reading configs from multiple sources
//! ### Simple coalesce
//! ```
//! use semigroup::Semigroup;
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(with = "semigroup::op::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::Overwrite")]
//!     pub boolean: bool,
//! }
//!
//! let cli = Config { num: Some(1), str: None, boolean: true };
//! let file = Config { num: None, str: Some("ten"), boolean: false };
//! let env = Config { num: Some(100), str: None, boolean: false };
//!
//! let config = cli.semigroup(file).semigroup(env);
//!
//! assert_eq!(config, Config { num: Some(1), str: Some("ten"), boolean: false });
//! ```
//!
//! ### Coalesce with rich enum annotation
//! Some [`Semigroup`] such as [`op::Coalesce`] can have an annotation.
//! More detail is in [`Annotate`].
//! ```
//! use semigroup::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::Overwrite")]
//!     pub boolean: bool,
//! }
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum Source {
//!     File,
//!     Env,
//!     Cli,
//! }
//!
//! let cli = Config { num: Some(1), str: None, boolean: true }.annotated(Source::Cli);
//! let file = Config { num: None, str: Some("ten"), boolean: false }.annotated(Source::File);
//! let env = Config { num: Some(100), str: None, boolean: false }.annotated(Source::Env);
//!
//! let config = cli.semigroup(file).semigroup(env);
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: false });
//! assert_eq!(config.annotation().num, Source::Cli);
//! assert_eq!(config.annotation().str, Source::File);
//! assert_eq!(config.annotation().boolean, Source::Env);
//! ```
//!
//! ## Statistically aggregation
//! ### Aggregate with histogram
//! Only available with the `histogram` feature. More detail is in [`op::HdrHistogram`].
//! ```
//! # #[cfg(feature="histogram")]
//! # {
//! use semigroup::{op::HdrHistogram, Semigroup};
//!
//! let histogram1 = (1..100).collect::<HdrHistogram<u32>>();
//! let histogram2 = (100..1000).collect::<HdrHistogram<u32>>();
//!
//! let histogram = histogram1.semigroup(histogram2);
//!
//! assert_eq!(histogram.mean(), 499.9999999999999);
//! assert_eq!(histogram.value_at_quantile(0.9), 900);
//! # }
//! ```
//!
//! ### Aggregate request-response result
//! Only available with the `histogram` feature.
//! ```
//! # #[cfg(all(feature="monoid", feature="histogram"))]
//! # {
//! use std::time::{Duration, Instant};
//! use semigroup::{
//!     op::{HdrHistogram, Sum, Min, Max},
//!     Construction, Commutative, Semigroup, OptionMonoid, Monoid
//! };
//!
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(commutative)]
//! pub struct RequestAggregate {
//!     count: Sum<u64>,
//!     pass: Sum<u64>,
//!     start: Min<Instant>,
//!     end: Max<Instant>,
//!     latency: HdrHistogram<u32>,
//! }
//! impl RequestAggregate {
//!     pub fn new(pass: bool, time: Instant, latency: Duration) -> Self {
//!         Self {
//!             count: Sum(1),
//!             pass: Sum(if pass { 1 } else { 0 }),
//!             start: Min(time),
//!             end: Max(time),
//!             latency: HdrHistogram::from_iter([latency.as_millis() as u64]),
//!         }
//!     }
//!     pub fn count(&self) -> u64 {
//!         self.count.into_inner()
//!     }
//!     pub fn pass_rate(&self) -> f64 {
//!         self.pass.into_inner() as f64 / self.count.into_inner() as f64
//!     }
//!     pub fn duration(&self) -> Duration {
//!         self.end.into_inner() - self.start.into_inner()
//!     }
//!     pub fn rps(&self) -> f64 {
//!         self.count.into_inner() as f64 / self.duration().as_secs_f64()
//!     }
//!     pub fn p99_latency(&self) -> Duration {
//!         Duration::from_millis(self.latency.value_at_quantile(0.99) as u64)
//!     }
//! }
//!
//! let (now, mut agg) = (Instant::now(), OptionMonoid::unit());
//! for i in 0..10000 {
//!     let duration = Duration::from_millis(i);
//!     agg = agg.semigroup(RequestAggregate::new(i % 2 == 0, now + duration, duration).into());
//! }
//!
//! let request_aggregate = agg.into_inner().unwrap();
//! assert_eq!(request_aggregate.count(), 10000);
//! assert_eq!(request_aggregate.pass_rate(), 0.5);
//! assert_eq!(request_aggregate.duration(), Duration::from_millis(9999));
//! assert_eq!(request_aggregate.rps(), 1000.1000100010001);
//! assert_eq!(request_aggregate.p99_latency(), Duration::from_millis(9903));
//! # }
//! ```
//!
//! ## Segment tree
//! More detail is in [`segment_tree::SegmentTree`] that requires [`Monoid`].
//! ### Range sum
//! Only available with the `monoid` feature
//! ```
//! # #[cfg(feature="monoid")]
//! # {
//! use semigroup::{op::Sum, Semigroup, Construction, segment_tree::SegmentTree};
//! let data = 0..=10000;
//! let mut sum_tree: SegmentTree<_> = data.into_iter().map(Sum).collect();
//! assert_eq!(sum_tree.fold(3..6).into_inner(), 12);
//! assert_eq!(sum_tree.fold(..).into_inner(), 50005000);
//! sum_tree.update_with(4, |Sum(x)| Sum(x + 50));
//! sum_tree.update_with(9999, |Sum(x)| Sum(x + 500000));
//! assert_eq!(sum_tree.fold(3..6).into_inner(), 62);
//! assert_eq!(sum_tree.fold(..).into_inner(), 50505050);
//! # }
//! ```
//! ### Custom monoid operator
//! Only available with the `monoid` feature
//! ```
//! # #[cfg(feature="monoid")]
//! # {
//! use semigroup::{Semigroup, Construction, segment_tree::SegmentTree, Monoid};
//! #[derive(
//!     Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Construction,
//! )]
//! #[construction(monoid, commutative, unit = Self(i32::MIN))]
//! struct Max(pub i32);
//! impl Semigroup for Max {
//!     fn op(base: Self, other: Self) -> Self {
//!         Self(std::cmp::max(base.0, other.0))
//!     }
//! }
//!
//! let data = [2, -5, 122, -33, -12, 14, -55, 500, 3];
//! let mut max_tree: SegmentTree<_> = data.into_iter().map(Max).collect();
//! assert_eq!(max_tree.fold(3..6).0, 14);
//! max_tree.update_with(4, |Max(x)| Max(x + 1000));
//! assert_eq!(max_tree.fold(3..6).0, 988);
//!
//! // #[test]
//! semigroup::assert_monoid!(&max_tree[..]);
//! # }
//! ```
//!
//! # Links
//! - GitHub: <https://github.com/hayas1/semigroup>
//! - GitHub Pages: <https://hayas1.github.io/semigroup/semigroup>
//! - Crates.io: <https://crates.io/crates/semigroup>
//! - Docs.rs: <https://docs.rs/semigroup>
//!
//! # Testing
//! ## Benchmarks
//! // TODO
//!
//! ## Coverage
//! <https://hayas1.github.io/semigroup/semigroup/tarpaulin-report.html>
//!

mod annotate;
mod commutative;
mod construction;
mod iter;
#[cfg(feature = "monoid")]
mod monoid;
pub mod op;
#[cfg(feature = "monoid")]
pub mod segment_tree;
mod semigroup;

pub use self::{annotate::*, commutative::*, construction::*, iter::*, semigroup::*};

#[cfg(feature = "monoid")]
pub use self::monoid::*;

#[cfg(feature = "derive")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use semigroup_derive::{properties, Construction, Semigroup};
