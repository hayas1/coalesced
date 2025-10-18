use quote::format_ident;
use syn::{parse_quote, DeriveInput, FieldValue, Fields, Ident, Member, Stmt};

use crate::{
    constant::Constant,
    semigroup::attr::{ContainerAttr, FieldAttr},
};

#[derive(Debug, Clone)]
pub struct FieldSemigroupOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
}
impl<'a> FieldSemigroupOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field_attr: FieldAttr,
    ) -> Self {
        Self {
            constant,
            container_attr,
            member,
            field_attr,
        }
    }
    pub fn new_fields(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        fields: &'a Fields,
    ) -> syn::Result<Vec<Self>> {
        fields
            .iter()
            .zip(fields.members())
            .map(|(field, member)| {
                Ok(Self::new(
                    constant,
                    derive,
                    container_attr,
                    member,
                    FieldAttr::new(field)?,
                ))
            })
            .collect()
    }

    pub fn impl_field_semigroup_op(&self) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    path_construction_trait,
                    ..
                },
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.map(|path| {
            parse_quote! {
                #member: <#path<_> as #path_construction_trait<_>>::lift_op(base.#member, other.#member)
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #member: #path_semigroup::op(base.#member, other.#member)
            }
        })
    }
    pub fn impl_field_monoid_unit(&self) -> FieldValue {
        let Self {
            constant:
                Constant {
                    path_monoid,
                    path_construction_monoid,
                    ..
                },
            container_attr,
            member,
            field_attr,
            ..
        } = self;
        let with = field_attr.with(container_attr);
        with.map(|path| {
            parse_quote! {
                #member: <#path<_> as #path_construction_monoid<_>>::lift_unit()
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                #member: #path_monoid::unit()
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct FieldAnnotatedOp<'a> {
    constant: &'a Constant,
    container_attr: &'a ContainerAttr,
    member: Member,
    field_attr: FieldAttr,
}
impl<'a> FieldAnnotatedOp<'a> {
    pub fn new(
        constant: &'a Constant,
        _derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        member: Member,
        field_attr: FieldAttr,
    ) -> Self {
        Self {
            constant,
            container_attr,
            member,
            field_attr,
        }
    }
    pub fn new_fields(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        container_attr: &'a ContainerAttr,
        fields: &'a Fields,
    ) -> syn::Result<Vec<Self>> {
        fields
            .iter()
            .zip(fields.members())
            .map(|(field, member)| {
                Ok(Self::new(
                    constant,
                    derive,
                    container_attr,
                    member,
                    FieldAttr::new(field)?,
                ))
            })
            .collect()
    }

    pub fn ident_variable(&self) -> Ident {
        match &self.member {
            Member::Named(ident) => ident.clone(),
            Member::Unnamed(index) => format_ident!("_{}", index.index),
        }
    }
    pub fn ident_parts(&self) -> (Ident, Ident) {
        let ident = self.ident_variable();
        (
            format_ident!("{}_value", ident),
            format_ident!("{}_annotation", ident),
        )
    }
    pub fn impl_field_annotated_op(&self) -> Stmt {
        let Self {
            constant,
            container_attr,
            member,
            field_attr,
        } = self;
        let Constant {
            path_annotated_semigroup,
            path_annotated,
            path_construction_annotated,
            ..
        } = constant;
        let (ident_value, ident_annotation) = self.ident_parts();
        let with = field_attr.with(container_attr);

        with.map(|path| {
            parse_quote! {
                let (#ident_value, #ident_annotation) = <#path::<_> as #path_construction_annotated<_, _>>::lift_annotated_op(
                    #path_annotated::new(base_value.#member, base_annotation.#member),
                    #path_annotated::new(other_value.#member, other_annotation.#member),
                ).into_parts();
            }
        })
        .unwrap_or_else(|| {
            parse_quote! {
                let (#ident_value, #ident_annotation) = #path_annotated_semigroup::annotated_op(
                    #path_annotated::new(base_value.#member, base_annotation.#member),
                    #path_annotated::new(other_value.#member, other_annotation.#member),
                ).into_parts();
            }
        })
    }
    pub fn impl_field_value(&self) -> FieldValue {
        let Self { member, .. } = self;
        let (ident_value, _ident_annotation) = self.ident_parts();
        parse_quote! {
            #member: #ident_value
        }
    }
    pub fn impl_field_annotation(&self) -> FieldValue {
        let Self { member, .. } = self;
        let (_ident_value, ident_annotation) = self.ident_parts();
        parse_quote! {
            #member: #ident_annotation
        }
    }
}
