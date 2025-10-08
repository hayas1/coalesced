use darling::FromDeriveInput;
use heck::ToSnakeCase;
use quote::format_ident;
use syn::{parse_quote, DeriveInput, Expr, Ident, TypeParam};

use crate::{
    annotation::Annotation,
    constant::Constant,
    error::{attr_name, ConstructionError},
};

#[derive(Debug, Clone, PartialEq, FromDeriveInput)]
#[darling(attributes(construction), and_then = Self::validate)]
pub struct ContainerAttr {
    #[darling(default)]
    annotated: bool,
    unit: Option<Expr>,

    #[darling(default)]
    commutative: bool,

    op_trait: Option<Ident>,

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
            unit,
            annotation_type_param,
            annotation_where,
            without_annotate_impl,
            ..
        } = &self;
        if !annotated {
            let err_attr_name = if unit.is_some() {
                Some(attr_name!(unit))
            } else if annotation_type_param.is_some() {
                Some(attr_name!(annotation_type_param))
            } else if annotation_where.is_some() {
                Some(attr_name!(annotation_where))
            } else if *without_annotate_impl {
                Some(attr_name!(without_annotate_impl))
            } else {
                None
            };
            err_attr_name.map_or(Ok(()), |a| {
                Err(darling::Error::custom(ConstructionError::OnlyAnnotated(a)))
            })?;
        }
        Ok(self)
    }

    pub fn is_annotated(&self) -> bool {
        self.annotated
    }

    pub fn op_trait(&self) -> Option<&Ident> {
        self.op_trait.as_ref()
    }
    pub fn op_method(&self, ident: &Ident) -> Option<Ident> {
        self.op_trait
            .as_ref()
            .map(|_| format_ident!("{}", ident.to_string().to_snake_case()))
    }

    pub fn is_commutative(&self) -> bool {
        self.commutative
    }

    pub fn unit_annotate(&self) -> Expr {
        self.unit.clone().unwrap_or_else(|| parse_quote!(()))
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
            #[construction(annotated, op_trait = CoalesceExt)]
            pub struct Coalesce<T>(pub Option<T>);
        },
        Ok(ContainerAttr {
            annotated: true,
            op_trait: Some(format_ident!("CoalesceExt")),
            ..default_container_attr()
        }),
    )]
    #[case::invalid_annotated_attr(
        syn::parse_quote! {
            #[derive(Construction)]
            #[construction(op_trait = CoalesceExt, unit = ())]
            pub struct Construct<T>(T);
        },
        Err("attribute `unit` are supported only with `annotated`"),
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
