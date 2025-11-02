use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, ItemImpl, parse_quote};

use crate::{annotation::Annotation, constant::Constant, construction::attr::ContainerAttr};

#[derive(Debug, Clone)]
pub struct OpTrait<'a> {
    constant: &'a Constant,
    derive: &'a DeriveInput,

    attr: &'a ContainerAttr,
    annotation: Annotation,
}
impl ToTokens for OpTrait<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.impl_monoid().to_tokens(tokens);
        self.impl_commutative().to_tokens(tokens);
        self.impl_semigroup_with_unit_annotation().to_tokens(tokens);
        self.impl_annotate().to_tokens(tokens);
    }
}
impl<'a> OpTrait<'a> {
    pub fn new(constant: &'a Constant, derive: &'a DeriveInput, attr: &'a ContainerAttr) -> Self {
        let annotation = attr.annotation(constant);

        Self {
            constant,
            derive,
            attr,
            annotation,
        }
    }

    pub fn impl_monoid(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            ..
        } = self;
        let Constant {
            path_monoid,
            attr_feature_monoid,
            ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let mut g = generics.clone();
        attr.monoid_where()
            .into_iter()
            .for_each(|w| g.make_where_clause().predicates.push(w));
        (attr.is_monoid() && attr.with_monoid_impl()).then(|| {
            attr.identity()
                .map(|expr| {
                    let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                #expr
                            }
                        }
                    }
                })
                .unwrap_or_else(|| {
                    let where_default = parse_quote! { Self: Default };
                    g.make_where_clause().predicates.push(where_default);
                    let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
                    parse_quote! {
                        #[automatically_derived]
                        #attr_feature_monoid
                        impl #impl_generics #path_monoid for #ident #ty_generics #where_clause {
                            fn identity() -> Self {
                                Default::default()
                            }
                        }
                    }
                })
        })
    }

    pub fn impl_commutative(&self) -> Option<ItemImpl> {
        let Self {
            constant,
            derive,
            attr,
            ..
        } = self;
        let Constant {
            path_commutative, ..
        } = constant;
        let DeriveInput {
            ident, generics, ..
        } = derive;
        let mut g = generics.clone();
        attr.commutative_where()
            .into_iter()
            .for_each(|w| g.make_where_clause().predicates.push(w));
        let (impl_generics, ty_generics, where_clause) = g.split_for_impl();
        attr.is_commutative().then(|| {
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_commutative for #ident #ty_generics #where_clause {}
            }
        })
    }

    pub fn impl_semigroup_with_unit_annotation(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_semigroup,
                    path_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            ..
        } = self;

        attr.is_annotated().then(|| {
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
            let unit_annotation = attr.unit_annotation();
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_semigroup for #ident #ty_generics #where_clause {
                    fn op(base: Self, other: Self) -> Self {
                        #path_annotated::lift_unit_annotated_op((base, #unit_annotation), (other, #unit_annotation))
                    }
                }
            }
        })
    }

    pub fn impl_annotate(&self) -> Option<ItemImpl> {
        let Self {
            constant:
                Constant {
                    path_annotate,
                    path_annotated,
                    ..
                },
            derive: DeriveInput {
                ident, generics, ..
            },
            attr,
            annotation,
            ..
        } = self;

        (attr.is_annotated() && attr.with_annotate_impl()).then(|| {
            let (_, ty_generics, _) = generics.split_for_impl();
            let (impl_generics, annotated_type, where_clause) = annotation.split_for_impl(generics);
            parse_quote! {
                #[automatically_derived]
                impl #impl_generics #path_annotate<#annotated_type> for #ident #ty_generics #where_clause {
                    type Annotation = #annotated_type;
                    fn annotated(self, annotation: Self::Annotation) -> #path_annotated<Self, #annotated_type> {
                        #path_annotated::new(
                            self,
                            annotation,
                        )
                    }
                }
            }
        })
    }
}
