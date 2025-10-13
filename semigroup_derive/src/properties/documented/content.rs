use syn::{parse_quote, Attribute, Expr, ExprLit, ItemStruct, Lit, Meta, MetaNameValue};

use crate::{
    constant::Constant,
    error::PropertiesError,
    properties::{attr::ContainerAttr, documented::table::PropertiesTable},
};

#[derive(Debug, Clone)]
pub struct Content<'a> {
    constant: &'a Constant,
    doc: String,
    table: PropertiesTable<'a>,
}
impl<'a> Content<'a> {
    pub fn new(
        constant: &'a Constant,
        attr: &'a ContainerAttr,
        item: &'a ItemStruct,
    ) -> syn::Result<Self> {
        Ok(Self {
            constant,
            doc: Self::doc(&item.attrs)?,
            table: PropertiesTable::new(constant, attr, item),
        })
    }
    pub fn doc(attrs: &[Attribute]) -> syn::Result<String> {
        Ok(attrs
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
                _ => Err(syn::Error::new_spanned(
                    attr,
                    PropertiesError::InvalidDocAttr,
                )),
            })
            .collect::<Result<Vec<_>, _>>()?
            .join("\n"))
    }
    pub fn embed_properties(&mut self) {
        let marker = "<!-- properties -->";
        let (start, end) = ("<!-- properties start -->", "<!-- properties end -->");
        self.doc = self
            .doc
            .replace(marker, &format!("{start}\n{}\n{end}", &self.table.table()));
    }
    pub fn to_attributes(&self) -> Vec<Attribute> {
        self.doc
            .split("\n")
            .map(|d| parse_quote! { #[doc = #d] })
            .collect()
    }
}
