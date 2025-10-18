use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{
    constant::{DERIVE_CONSTRUCTION, DERIVE_SEMIGROUP},
    name::Name,
};

#[derive(Debug, Clone)]
pub enum ConstructionError {
    OnlyNewType,
    OnlyAnnotated(Name),
    OnlyMonoid(Name),
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
            Self::OnlyAnnotated(Name(name)) => {
                write!(f, "attribute `{name}` are supported only with `annotated`")
            }
            Self::OnlyMonoid(Name(name)) => {
                write!(f, "attribute `{name}` are supported only with `monoid`")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SemigroupError {
    UnsupportedEnum,
    UnsupportedUnion,
    OnlyAnnotated(Name),
    OnlyMonoid(Name),
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
            Self::OnlyAnnotated(Name(name)) => {
                write!(f, "attribute `{name}` are supported only with `annotated`")
            }
            Self::OnlyMonoid(Name(name)) => {
                write!(f, "attribute `{name}` are supported only with `monoid`")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertiesError {
    InvalidDocAttr,
}
impl Error for PropertiesError {}
impl Display for PropertiesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDocAttr => write!(f, "invalid doc attribute"),
        }
    }
}
