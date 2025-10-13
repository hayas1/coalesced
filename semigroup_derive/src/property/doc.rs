use syn::{parse_quote, Attribute, Expr, ExprLit, Lit, Meta, MetaNameValue};

use crate::error::PropertyError;

#[derive(Debug, Clone, PartialEq)]
pub struct DocAttr {
    doc: String,
}
impl DocAttr {
    pub fn new(attrs: &[Attribute]) -> syn::Result<Self> {
        let doc = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .map(|attr| match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(str), ..
                        }),
                    ..
                }) => Ok(str.value()),
                _ => Err(syn::Error::new_spanned(attr, PropertyError::InvalidDocAttr)),
            })
            .collect::<Result<Vec<_>, _>>()?
            .join("\n");
        Ok(Self { doc })
    }
    pub fn embed_properties<S: Into<String>>(&mut self, content: S) {
        let marker = "<!-- properties -->";
        let (start, end) = ("<!-- properties start -->", "<!-- properties end -->");
        self.doc = self
            .doc
            .replace(marker, &format!("{start}\n{}\n{end}", &content.into()));
    }
    pub fn to_attributes(&self) -> Vec<Attribute> {
        self.doc
            .split("\n")
            .map(|d| parse_quote! { #[doc = #d] })
            .collect()
    }
}
