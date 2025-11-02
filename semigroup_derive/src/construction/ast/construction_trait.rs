use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Field, ItemImpl, parse_quote};

use crate::{annotation::Annotation, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct ConstructionTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,
    attr: &'a ContainerAttr,
    annotation: Annotation,

    field: &'a Field,
}
impl ToTokens for ConstructionTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_from().to_tokens(tokens);
        self.impl_deref().to_tokens(tokens);
        self.impl_deref_mut().to_tokens(tokens);
        self.impl_construction().to_tokens(tokens);
        self.impl_construction_annotated().to_tokens(tokens);
        self.impl_construction_monoid().to_tokens(tokens);
    }
}
impl<'a> ConstructionTrait<'a> {
    pub fn new(
        constant: &'a Constant,
        derive: &'a DeriveInput,
        attr: &'a ContainerAttr,
        field: &'a Field,
    ) -> syn::Result<Self> {
        let annotation = attr.annotation(constant);

        Ok(Self {
            constant,
            derive,
            attr,
            annotation,
            field,
        })
    }
    pub fn impl_from(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics From<#ty> for #ident #ty_generics #where_clause {
                fn from(value: #ty) -> Self {
                    #ident(value)
                }
            }
        }
    }
    pub fn impl_construction(&self) -> ItemImpl {
        let Self {
            constant:
                Constant {
                    path_construction_trait,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics #path_construction_trait<#ty> for #ident #ty_generics #where_clause {
                fn into_inner(self) -> #ty {
                    self.0
                }
            }
        }
    }
    pub fn impl_construction_annotated(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_construction_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            attr,
            annotation,
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_construction_annotated<#ty, #annotated_type> for #ident #ty_generics #where_clause {}
            }
        })
    }
    pub fn impl_construction_monoid(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_monoid,
                    path_construction_monoid,
                    attr_feature_monoid,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            attr,
            ..
        } = self;

        attr.is_monoid().then(|| {
            let mut g = generics.clone();
            g.make_where_clause().predicates.push(parse_quote!{ Self: #path_monoid });
            let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
            parse_quote! {
                #[automatically_derived]
                #attr_feature_monoid
                impl #impl_generics #path_construction_monoid<#ty> for #ident #ty_generics #where_clause {}
            }
        })
    }
    pub fn impl_deref(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            field: Field { ty, .. },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics std::ops::Deref for #ident #ty_generics #where_clause {
                type Target = #ty;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }
    pub fn impl_deref_mut(&self) -> ItemImpl {
        let Self {
            derive: DeriveInput {
                ident, generics, ..
            },
            ..
        } = self;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        parse_quote! {
            #[automatically_derived]
            impl #impl_generics std::ops::DerefMut for #ident #ty_generics #where_clause {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
    }
}
