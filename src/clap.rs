use std::str::FromStr;

use crate::Coalesced;

// TODO not Option<T>, T instead ?
impl<T, P> FromStr for Coalesced<Option<T>, P>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Coalesced::new(None))
        } else {
            s.parse().map(Some).map(Coalesced::new)
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Debug, Clone, Parser)]
    #[clap(version, about, arg_required_else_help = true)]
    pub struct Cli {
        /// Some number
        #[arg(short, long)]
        pub number: Coalesced<Option<i64>>,
    }

    #[test]
    fn test_clap_parser() {
        let cli = Cli::try_parse_from(["coalesced", "--number", "100"]).unwrap();
        assert_eq!(cli.number.unwrap(), 100);

        let cli = Cli::try_parse_from(["coalesced"]).unwrap();
        assert!(cli.number.is_none());
    }
}
