use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::constant::{DERIVE_CONSTRUCTION, DERIVE_SEMIGROUP};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    OnlyNewType,
    OnlyAnnotated(AttrName),
}
impl Error for ConstructionError {}
impl Display for ConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnlyNewType => {
                write!(
                    f,
                    "derive {DERIVE_CONSTRUCTION} only supports newtype structs",
                )
            }
            Self::OnlyAnnotated(AttrName(name)) => {
                write!(f, "attribute `{name}` are supported only with `annotated`")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemigroupError {
    UnsupportedEnum,
    UnsupportedUnion,
    OnlyAnnotated(AttrName),
}
impl Error for SemigroupError {}
impl Display for SemigroupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedEnum => {
                write!(f, "derive {DERIVE_SEMIGROUP} does not support enums")
            }
            Self::UnsupportedUnion => {
                write!(f, "derive {DERIVE_SEMIGROUP} does not support unions")
            }
            Self::OnlyAnnotated(AttrName(name)) => {
                write!(f, "attribute `{name}` are supported only with `annotated`")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertyError {
    InvalidDocAttr,
}
impl Error for PropertyError {}
impl Display for PropertyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDocAttr => write!(f, "invalid doc attribute"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrName(pub &'static str);
macro_rules! attr_name {
    ($var:ident) => {{
        let _ = &$var; // to follow the renaming
        crate::error::AttrName(stringify!($var))
    }};
}
pub(crate) use attr_name;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_name() {
        let foo = ();
        assert_eq!(attr_name!(foo), AttrName("foo"));
        let bar = ();
        assert_eq!(attr_name!(bar), AttrName("bar"));
        let baz = ();
        assert_eq!(attr_name!(baz), AttrName("baz"));
    }
}
