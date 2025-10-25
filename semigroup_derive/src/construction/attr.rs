use darling::FromDeriveInput;
use syn::{parse_quote, DeriveInput, Expr, TypeParam, WherePredicate};

use crate::{annotation::Annotation, constant::Constant, error::ConstructionError, name::var_name};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(construction), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,
    unit_annotation: Option<Expr>,

    #[darling(default)]
    monoid: bool,
    unit: Option<Expr>,
    unit_where: Option<String>, // TODO Vec
    #[darling(default)]
    without_monoid_impl: bool,

    #[darling(default)]
    commutative: bool,
    commutative_where: Option<String>, // TODO Vec

    annotation_type_param: Option<TypeParam>,
    annotation_where: Option<String>, // TODO Vec
    #[darling(default)]
    without_annotate_impl: bool,

    #[darling(default)]
    without_construction: bool,
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
            unit_where,
            without_monoid_impl: with_monoid_impl,
            commutative,
            commutative_where,
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
            } else if unit_where.is_some() {
                Some(var_name!(unit_where))
            } else if *with_monoid_impl {
                Some(var_name!(with_monoid_impl))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyMonoid(a)))
            })?;
        }
        if !commutative {
            let err_attr_name = if commutative_where.is_some() {
                Some(var_name!(commutative_where))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyCommutative(
                    a,
                )))
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
    pub fn unit_where(&self) -> Option<WherePredicate> {
        self.unit_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
    }
    pub fn with_monoid_impl(&self) -> bool {
        !self.without_monoid_impl
    }

    pub fn is_commutative(&self) -> bool {
        self.commutative
    }
    pub fn commutative_where(&self) -> Option<WherePredicate> {
        self.commutative_where
            .as_deref()
            .map(syn::parse_str)
            .map(|p| p.unwrap_or_else(|e| todo!("{e}")))
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
                .as_deref()
                .map(syn::parse_str)
                .map(|p| p.unwrap_or_else(|e| todo!("{e}"))),
        )
    }
    pub fn with_annotate_impl(&self) -> bool {
        !self.without_annotate_impl
    }

    pub fn with_construction(&self) -> bool {
        !self.without_construction
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
    #[case::invalid_monoid_attr(
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(unit = ())]
            pub struct Construct<T>(T);
        },
        Err("attribute `unit` are supported only with `monoid`"),
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
