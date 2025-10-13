use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ItemStruct;

use crate::{
    constant::Constant,
    properties::{attr::ContainerAttr, documented::content::Content},
};

mod content;
mod table;

#[derive(Debug, Clone)]
pub struct Documented<'a> {
    content: Content<'a>,
    item: &'a ItemStruct,
}
impl ToTokens for Documented<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_item = ItemStruct {
            attrs: self.content.to_attributes(),
            ..self.item.clone()
        };
        new_item.to_tokens(tokens);
    }
}
impl<'a> Documented<'a> {
    pub fn new(
        constant: &'a Constant,
        attr: &'a ContainerAttr,
        item: &'a ItemStruct,
    ) -> syn::Result<Self> {
        let mut content = Content::new(constant, attr, item)?;
        content.embed_properties();

        Ok(Self { content, item })
    }
}
