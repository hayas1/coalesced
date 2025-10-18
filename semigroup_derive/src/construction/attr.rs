use darling::FromDeriveInput;
use syn::{parse_quote, DeriveInput, Expr, TypeParam, WhereClause};

use crate::{annotation::Annotation, constant::Constant, error::ConstructionError, name::var_name};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(construction), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,
    unit_annotation: Option<Expr>,

    #[darling(default)]
    monoid: bool,
    #[darling(default)]
    unit: Option<Expr>,

    #[darling(default)]
    commutative: bool,

    annotation_type_param: Option<TypeParam>,
    annotation_where: Option<String>,
    #[darling(default)]
    without_annotate_impl: bool,
}
impl ContainerAttr {
    pub fn new(derive: &DeriveInput) -> syn::Result<Self> {
        Ok(Self::from_derive_input(derive)?)
    }
    pub fn validate(self) -> darling::Result<Self> {
        let Self {
            annotated,
            unit_annotation,
            annotation_type_param,
            annotation_where,
            without_annotate_impl,
            monoid,
            unit,
            ..
        } = &self;
        if !annotated {
            let err_attr_name = if unit_annotation.is_some() {
                Some(var_name!(unit_annotation))
            } else if annotation_type_param.is_some() {
                Some(var_name!(annotation_type_param))
            } else if annotation_where.is_some() {
                Some(var_name!(annotation_where))
            } else if *without_annotate_impl {
                Some(var_name!(without_annotate_impl))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyAnnotated(a)))
            })?;
        }
        if !monoid {
            let err_attr_name = if unit.is_some() {
                Some(var_name!(unit))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyMonoid(a)))
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

    pub fn unit_annotate(&self) -> Expr {
        self.unit_annotation
            .clone()
            .unwrap_or_else(|| parse_quote!(()))
    }

    pub fn annotation(&self, constant: &Constant) -> Annotation {
        Annotation::new(
            self.annotation_type_param
                .as_ref()
                .unwrap_or(&constant.default_type_param)
                .clone(),
            None,
            self.annotation_where
                .as_ref()
                .map(|p| syn::parse_str(p).unwrap_or_else(|_| todo!())),
        )
    }
    pub fn with_annotate_impl(&self) -> bool {
        !self.without_annotate_impl
    }

    pub fn push_monoid_where(&self, where_clause: &mut WhereClause) {
        self.is_monoid().then(|| {
            where_clause.predicates.push(parse_quote! {
                Self: Default
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn default_container_attr() -> ContainerAttr {
        ContainerAttr::new(&parse_quote! {
            #[derive(Construction)]
            pub struct Construct<T>(T);
        })
        .unwrap()
    }

    #[rstest]
    #[case::ok(
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(annotated)]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            annotated: true,
            ..default_container_attr()
        }),
    )]
    #[case::invalid_annotated_attr(
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(unit_annotation = ())]
            pub struct Construct<T>(T);
        },
        Err("attribute `unit_annotation` are supported only with `annotated`"),
    )]
    fn test_construction_container_attr(
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
