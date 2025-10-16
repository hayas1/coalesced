#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! [`Semigroup`] trait is useful for
//! - reading configs from multiple sources
//! - statistically aggregation
//! - fast range queries using segment tree
//!
//! # Usage
//! ```toml
//! [dependencies]
//! semigroup = { git = "https://github.com/hayas1/semigroup", features = ["derive", "monoid"] }
//! ```
//!
//! # Examples
//!
//! ## Reading configs from multiple sources
//! ### Simple coalesce
//! ```
//! use semigroup::Semigroup;
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(with = "semigroup::op::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::overwrite::Overwrite")]
//!     pub boolean: bool,
//! }
//!
//! let file = Config { num: Some(1), str: None, boolean: true };
//! let env = Config { num: None, str: Some("ten"), boolean: false };
//! let cli = Config { num: Some(100), str: None, boolean: false };
//!
//! let config = file.semigroup(env).semigroup(cli);
//!
//! assert_eq!(config, Config { num: Some(1), str: Some("ten"), boolean: false });
//! ```
//!
//! ### Coalesce with rich enum annotation
//! ```
//! use semigroup::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::overwrite::Overwrite")]
//!     pub boolean: bool,
//! }
//! #[derive(Debug, Clone, PartialEq)]
//! pub enum Source {
//!     File,
//!     Env,
//!     Cli,
//! }
//!
//! let file = Config { num: Some(1), str: None, boolean: true }.annotated(Source::File);
//! let env = Config { num: None, str: Some("ten"), boolean: false }.annotated(Source::Env);
//! let cli = Config { num: Some(100), str: None, boolean: false }.annotated(Source::Cli);
//!
//! let config = file.semigroup(env).semigroup(cli);
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: false });
//! assert_eq!(config.annotation().num, Source::File);
//! assert_eq!(config.annotation().str, Source::Env);
//! assert_eq!(config.annotation().boolean, Source::Cli);
//! ```
//!
//! ## Statistically aggregation
//! ### Aggregate with histogram
//! Only available with the `histogram` feature
//! ```
//! # #[cfg(feature="histogram")]
//! # {
//! use semigroup::{op::hdr_histogram::HdrHistogram, Semigroup};
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
//! ## Segment tree
//! More detail [`crate::segment_tree::SegmentTree`]
//! ### Range sum
//! Only available with the `monoid` feature
//! ```
//! # #[cfg(feature="monoid")]
//! # {
//! use semigroup::{op::sum::Sum, Semigroup, Construction, segment_tree::SegmentTree};
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
//! struct Max(pub i32);
//! impl Semigroup for Max {
//!     fn op(base: Self, other: Self) -> Self {
//!         Max(std::cmp::max(base.0, other.0))
//!     }
//! }
//! impl Monoid for Max {
//!     fn unit() -> Self {
//!         Max(i32::MIN)
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
//! # Documents
//! <https://hayas1.github.io/semigroup/semigroup>
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
