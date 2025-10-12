#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! Semigroup trait is useful
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
//! ### With simple string annotation
//! ```
//! use semigroup::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
//!     pub boolean: bool,
//! }
//!
//! let file = Config { num: Some(1), str: None, boolean: true }.annotated("File");
//! let env = Config { num: None, str: Some("ten"), boolean: false }.annotated("Env");
//! let cli = Config { num: Some(100), str: None, boolean: true }.annotated("Cli");
//!
//! let config = file.semigroup(env).semigroup(cli);
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
//! assert_eq!(config.annotation().num, "File");
//! assert_eq!(config.annotation().str, "Env");
//! assert_eq!(config.annotation().boolean, "Cli");
//! ```
//!
//! ### With rich enum annotation
//! ```
//! use semigroup::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
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
//! let cli = Config { num: Some(100), str: None, boolean: true }.annotated(Source::Cli);
//!
//! let config = file.semigroup(env).semigroup(cli);
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
//! assert_eq!(config.annotation().num, Source::File);
//! assert_eq!(config.annotation().str, Source::Env);
//! assert_eq!(config.annotation().boolean, Source::Cli);
//! ```
//!
//! ## Lazy Evaluation
//! ### Reduce
//! ```
//! use semigroup::{Annotate, Semigroup};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
//!     pub boolean: bool,
//! }
//!
//! let lazy = vec![
//!     Config { num: Some(1), str: None, boolean: true }.annotated("File"),
//!     Config { num: None, str: Some("ten"), boolean: false }.annotated("Env"),
//!     Config { num: Some(100), str: None, boolean: true }.annotated("Cli"),
//! ];
//!
//!
//! let config = lazy.into_iter().reduce(|acc, item| acc.semigroup(item));
//!
//! assert_eq!(config.as_ref().unwrap().value(), &Config { num: Some(1), str: Some("ten"), boolean: true });
//! assert_eq!(config.as_ref().unwrap().annotation().num, "File");
//! assert_eq!(config.as_ref().unwrap().annotation().str, "Env");
//! assert_eq!(config.as_ref().unwrap().annotation().boolean, "Cli");
//! ```
//! ### Fold with final default
//! ```
//! use semigroup::{Annotate, Semigroup, SemigroupIterator};
//! #[derive(Debug, Clone, PartialEq, Semigroup)]
//! #[semigroup(annotated, with = "semigroup::op::annotation::coalesce::Coalesce")]
//! pub struct Config<'a> {
//!     pub num: Option<u32>,
//!     pub str: Option<&'a str>,
//!     #[semigroup(with = "semigroup::op::annotation::overwrite::Overwrite")]
//!     pub boolean: bool,
//! }
//!
//! let lazy = vec![
//!     Config { num: Some(1), str: None, boolean: true }.annotated("File"),
//!     Config { num: None, str: None, boolean: false }.annotated("Env"),
//!     Config { num: Some(100), str: None, boolean: true }.annotated("Cli"),
//! ];
//!
//!
//! let config = lazy.into_iter().fold_final(Config { num: Some(1000), str: Some("thousand"), boolean: true }.annotated("Default"));
//!
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("thousand"), boolean: true });
//! assert_eq!(config.annotation().num, "File");
//! assert_eq!(config.annotation().str, "Default");
//! assert_eq!(config.annotation().boolean, "Default");
//! ```
//!
//! ## Segment tree
//! Only available with the `monoid` feature
//! ```
//! # #[cfg(feature="monoid")]
//! # {
//! use semigroup::{Semigroup, Construction, segment_tree::SegmentTree, monoid::Monoid};
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

pub use semigroup_base::{
    annotate::{Annotate, Annotated},
    commutative::{Commutative, Reverse},
    iter::{SemigroupDoubleEndedIterator, SemigroupIterator},
    op,
    semigroup::{AnnotatedSemigroup, Semigroup},
};

#[cfg(feature = "derive")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use semigroup_derive::{Construction, Semigroup};

#[cfg(feature = "test")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "test")))]
pub use semigroup_base::{monoid::tests::assert_monoid, semigroup::tests::assert_semigroup_op};

#[cfg(feature = "monoid")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "monoid")))]
pub use semigroup_base::{monoid, segment_tree};
