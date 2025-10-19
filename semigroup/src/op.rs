mod coalesce;
mod concat;
mod overwrite;
pub use {coalesce::*, concat::*, overwrite::*};

#[cfg(feature = "monoid")]
mod gcd;
#[cfg(feature = "monoid")]
mod lcm;
mod max;
mod min;
mod prod;
mod sum;
mod xor;
#[cfg(feature = "monoid")]
pub use {gcd::*, lcm::*};
pub use {max::*, min::*, prod::*, sum::*, xor::*};

#[cfg(feature = "histogram")]
mod hdr_histogram;
#[cfg(feature = "histogram")]
pub use hdr_histogram::*;
