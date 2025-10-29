#![cfg_attr(doc_cfg, feature(doc_cfg))]
//! [`Semigroup`](`crate::semigroup::Semigroup`) trait is useful for
//! - reading configs from multiple sources using [`Coalesce`](`crate::op::Coalesce`)
//! - statistically aggregation using [`Histogram`](`crate::op::HdrHistogram`)
//! - fast range queries using [`SegmentTree`](`crate::segment_tree::SegmentTree`)
//!
//! # Usage
//! ```sh
//! cargo add semigroup --features derive,monoid
//! ```
//!
//! # Examples
//! A CLI example of `clap` and `serde` integration, see <https://github.com/hayas1/semigroup/blob/master/semigroup/examples/clap_serde.rs>
//!
//! ## Simple coalesce
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
//! ## Coalesce with rich enum annotation and lazy evaluation
//! More detail is in [`Annotate`] and [`Lazy`].
//! ```
//! use semigroup::{Annotate, Lazy, Semigroup};
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
//! let lazy = Lazy::from(cli).semigroup(file.into()).semigroup(env.into());
//! assert_eq!(lazy.first().value(), &Config { num: Some(1), str: None, boolean: true });
//! assert_eq!(lazy.last().value(), &Config { num: Some(100), str: None, boolean: false });
//!
//! let config = lazy.combine();
//! assert_eq!(config.value(), &Config { num: Some(1), str: Some("ten"), boolean: false });
//! assert_eq!(config.annotation().num, Source::Cli);
//! assert_eq!(config.annotation().str, Source::File);
//! assert_eq!(config.annotation().boolean, Source::Env);
//! ```
//!
//! # Highlights
//! - [`Semigroup`] trait
//!   - derive [`Construction`] defines a new *semigroup* operation (Some operations are already defined in [`crate::op`]).
//!   - derive [`Semigroup`] implements *semigroup* by existing *semigroup* operation.
//!   - test *associativity* using [`assert_semigroup!`].
//! - Some related traits also supported by derive
//!   - [`Annotate`] supports practical *annotation*.
//!   - [`Monoid`] has *identity element*.
//!   - [`Commutative`] represents *commutativity*.
//! - Combine operations
//!   - [`CombineIterator`] provides *fold* and *combine* operations for iterators.
//!   - [`Lazy`] provides *lazy evaluation*.
//!   - [`segment_tree::SegmentTree`] is useful for fast range queries on [`Monoid`].
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
mod combine;
mod commutative;
mod construction;
#[cfg(feature = "monoid")]
mod monoid;
pub mod op;
#[cfg(feature = "monoid")]
pub mod segment_tree;
mod semigroup;

pub use self::{annotate::*, combine::*, commutative::*, construction::*, semigroup::*};

#[cfg(feature = "monoid")]
pub use self::monoid::*;

#[cfg(feature = "derive")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "derive")))]
pub use semigroup_derive::{properties, Construction, Semigroup};
