use darling::{FromDeriveInput, FromField};
use syn::{parse_quote, DeriveInput, Expr, Field, Ident, Path, WhereClause};

use crate::{annotation::Annotation, constant::Constant, error::SemigroupError, name::var_name};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(semigroup), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,

    #[darling(default)]
    monoid: bool,
    #[darling(default)]
    unit: Option<Expr>,

    #[darling(default)]
    commutative: bool,

    with: Option<Path>,
    annotation_param: Option<Ident>,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            annotated,
            annotation_param,
            monoid,
            unit,
            ..
        } = &self;
        if !annotated {
            let err_attr_name = if annotation_param.is_some() {
                Some(var_name!(annotation_param))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(SemigroupError::OnlyAnnotated(a)))
            })?;
        }
        if !monoid {
            let err_attr_name = if unit.is_some() {
                Some(var_name!(unit))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(SemigroupError::OnlyMonoid(a)))
            })?;
        }
        Ok(self)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated
    }

    pub fn is_monoid(&self) -> bool {
        self.monoid
    }
    pub fn unit(&self) -> Option<&Expr> {
        self.unit.as_ref()
    }

    pub fn is_commutative(&self) -> bool {
        self.commutative
    }

    pub fn annotation(&self, constant: &Constant, annotation_ident: &Ident) -> Annotation {
        let a = self
            .annotation_param
            .as_ref()
            .unwrap_or(&constant.default_type_param.ident);
        Annotation::new(
            parse_quote! { #a: Clone },
            Some(parse_quote! { #annotation_ident<#a> }),
            None,
        )
    }

    pub fn push_monoid_where(&self, where_clause: &mut WhereClause) {
        self.is_monoid().then(|| {
            where_clause.predicates.push(parse_quote! {
                Self: Default
            });
        });
    }
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(semigroup))]
pub struct FieldAttr {
    with: Option<Path>,
}
impl FieldAttr {
    pub fn new(field: &Field) -> syn::Result<Self> {
        Ok(Self::from_field(field)?)
    }
    pub fn with<'a>(&'a self, container: &'a ContainerAttr) -> Option<&'a Path> {
        self.with.as_ref().or(container.with.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn default_container_attr() -> ContainerAttr {
        ContainerAttr::new(&parse_quote! {
            #[derive(Semigroup)]
            pub struct NamedStruct {}
        })
        .unwrap()
    }

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotated)]
            pub struct NamedStruct {}
        },
        Ok(ContainerAttr {
            annotated: true,
            ..default_container_attr()
        }),
    )]
    #[case::invalid_annotated_attr(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(annotation_param = "X")]
            pub struct UnnamedStruct();
        },
        Err("attribute `annotation_param` are supported only with `annotated`"),
    )]
    #[case::invalid_monoid_attr(
        syn::parse_quote! {
            #[derive(Semigroup)]
            #[semigroup(unit = ())]
            pub struct UnnamedStruct();
        },
        Err("attribute `unit` are supported only with `monoid`"),
    )]
    fn test_semigroup_container_attr(
        #[case] input: DeriveInput,
        #[case] expected: Result<ContainerAttr, &str>,
    ) {
        let actual = ContainerAttr::new(&input);
        assert_eq!(
            actual.as_ref().map_err(ToString::to_string),
            expected.as_ref().map_err(ToString::to_string),
        );
    }
}
