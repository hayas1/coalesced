use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, DataEnum, DataStruct, DataUnion, DeriveInput, Field, Fields, FieldsUnnamed};

use crate::{
    constant::Constant,
    construction::{
        ast::{construction_trait::ConstructionTrait, op_trait::OpTrait},
        attr::ContainerAttr,
    },
    error::ConstructionError,
};

pub mod construction_trait;
pub mod op_trait;

#[derive(Debug, Clone)]
pub struct Construction<'a> {
    construction_trait: Option<ConstructionTrait<'a>>,
    op_trait: OpTrait<'a>,
}
impl ToTokens for Construction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.op_trait.to_tokens(tokens);
        self.construction_trait.to_tokens(tokens);
    }
}
impl<'a> Construction<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
    ) -> syn::Result<Self> {
        let construction_trait = attr
            .with_construction()
            .then(|| ConstructionTrait::new(constant, derive, attr, Self::newtype_field(derive)?))
            .transpose()?;
        let op_trait = OpTrait::new(constant, derive, attr);
        Ok(Self {
            construction_trait,
            op_trait,
        })
    }

    pub fn newtype_field(derive: &'a DeriveInput) -> syn::Result<&'a Field> {
        match &derive.data {
            Data::Struct(DataStruct {
                fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
                ..
            }) if unnamed.len() == 1 => {
                // if let [field] = unnamed => // match: `if let` guards are experimental see issue #51114 https://github.com/rust-lang/rust/issues/51114
                if let &[field] = unnamed.iter().collect::<Vec<_>>().as_slice() {
                    Ok(field)
                } else {
                    unreachable!()
                }
            }
            Data::Enum(DataEnum { enum_token, .. }) => Err(syn::Error::new_spanned(
                enum_token,
                ConstructionError::ExpectNewType,
            )),
            Data::Struct(DataStruct { struct_token, .. }) => Err(syn::Error::new_spanned(
                struct_token,
                ConstructionError::ExpectNewType,
            )),
            Data::Union(DataUnion { union_token, .. }) => Err(syn::Error::new_spanned(
                union_token,
                ConstructionError::ExpectNewType,
            )),
        }
    }
}
