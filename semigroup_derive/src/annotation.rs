use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    GenericParam, Generics, ImplGenerics, Type, TypeParam, WhereClause, WherePredicate, parse_quote,
};

#[derive(Debug, Clone)]
pub struct Annotation {
    type_param: TypeParam,
    ty: Type,
    generics: Generics,
}
impl Annotation {
    pub fn new(
        type_param: TypeParam,
        ty: Option<Type>,
        where_predicate: Option<WherePredicate>,
    ) -> Self {
        let ty = ty.unwrap_or_else(|| {
            let TypeParam { ident, .. } = &type_param;
            parse_quote! { #ident }
        });

        let mut generics = Generics::default();
        generics.params.push(GenericParam::Type(type_param.clone()));
        generics
            .make_where_clause()
            .predicates
            .extend(where_predicate);

        Self {
            type_param,
            ty,
            generics,
        }
    }

    pub fn param(&self) -> &TypeParam {
        &self.type_param
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn split_for_impl<'a>(
        &'a self,
        generics: &'a Generics,
    ) -> (
        AnnotationImplGenerics<'a>,
        AnnotationType<'a>,
        AnnotationWhereClause<'a>,
    ) {
        (
            AnnotationImplGenerics {
                annotation: self,
                generics,
            },
            AnnotationType { annotation: self },
            AnnotationWhereClause {
                annotation: self,
                generics,
            },
        )
    }
}

pub struct AnnotationImplGenerics<'a> {
    annotation: &'a Annotation,
    generics: &'a Generics,
}
impl<'a> ToTokens for AnnotationImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.generics.clone();
        self.impl_generics(&mut generics).to_tokens(tokens);
    }
}
impl<'a> AnnotationImplGenerics<'a> {
    pub fn impl_generics(&self, generics: &'a mut Generics) -> ImplGenerics<'a> {
        let generic_param = GenericParam::Type(self.annotation.param().clone());
        generics.params.push(generic_param);
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics
    }
}

pub struct AnnotationType<'a> {
    annotation: &'a Annotation,
}
impl<'a> ToTokens for AnnotationType<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.annotation.ty().to_tokens(tokens);
    }
}

pub struct AnnotationWhereClause<'a> {
    annotation: &'a Annotation,
    generics: &'a Generics,
}
impl<'a> ToTokens for AnnotationWhereClause<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut generics = self.generics.clone();
        let where_clause = generics.make_where_clause();
        self.extend_where_clause(where_clause);
        where_clause.to_tokens(tokens);
    }
}
impl AnnotationWhereClause<'_> {
    pub fn extend_where_clause(&self, where_clause: &mut WhereClause) {
        let mut annotation_generics = self.annotation.generics.clone();
        let annotation_where = annotation_generics.make_where_clause();
        where_clause
            .predicates
            .extend(annotation_where.predicates.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_without_type() {
        let syn::DeriveInput { generics, .. } = parse_quote! {
            struct Construction<T> (T) where T: Clone;
        };

        let annotation = Annotation::new(parse_quote! { A }, None, None);
        let (impl_generics, annotation_type, where_clause) = annotation.split_for_impl(&generics);
        assert_eq!(
            impl_generics.into_token_stream().to_string(),
            quote::quote! { <T, A> }.to_string(),
        );
        assert_eq!(
            annotation_type.into_token_stream().to_string(),
            quote::quote! { A }.to_string(),
        );
        assert_eq!(
            where_clause.into_token_stream().to_string(),
            quote::quote! { where T: Clone }.to_string(),
        );
    }

    #[test]
    fn test_annotation_with_type() {
        let syn::DeriveInput { generics, .. } = parse_quote! {
            struct NamedStruct<T> where T: Clone {
                name: String,
                value: Option<T>,
            }
        };

        let annotation = Annotation::new(
            parse_quote! { A },
            Some(parse_quote! { NamedStructAnnotation<A> }),
            Some(parse_quote! { A: Clone }),
        );
        let (impl_generics, annotation_type, where_clause) = annotation.split_for_impl(&generics);
        assert_eq!(
            impl_generics.into_token_stream().to_string(),
            quote::quote! { <T, A> }.to_string(),
        );
        assert_eq!(
            annotation_type.into_token_stream().to_string(),
            quote::quote! { NamedStructAnnotation<A> }.to_string(),
        );
        assert_eq!(
            where_clause.into_token_stream().to_string(),
            quote::quote! { where T: Clone, A: Clone }.to_string(),
        );
    }
}
