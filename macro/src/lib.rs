use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput};

fn get_list_attribute_token_stream(
    ast: &DeriveInput,
    name: &'static str,
) -> proc_macro2::TokenStream {
    struct ListArgs(proc_macro2::TokenStream);

    struct ListParser;

    impl Parser for ListParser {
        type Output = ListArgs;

        fn parse2(self, tokens: proc_macro2::TokenStream) -> syn::Result<Self::Output> {
            Ok(ListArgs(tokens))
        }
    }

    let font_size_attribute: &syn::Attribute = ast
        .attrs
        .iter()
        .filter(|x| x.path().is_ident(name))
        .next()
        .expect(format!("Could not find `{name}` attribute").as_str());

    let meta_list = font_size_attribute
        .meta
        .require_list()
        .expect(format!("`{name}` attribute must be a list").as_str());
    let l: ListArgs = meta_list
        .parse_args_with(ListParser)
        .expect(format!("Could not parse `{name}` args").as_str());

    l.0
}


#[proc_macro_derive(LayoutPositioning, attributes(width, height, left, top))]
pub fn derive_layout_positioning(input: TokenStream) -> TokenStream{
    let ast = parse_macro_input!(input as DeriveInput);

    let width = get_list_attribute_token_stream(&ast, "width");
    let height = get_list_attribute_token_stream(&ast, "height");
    let left = get_list_attribute_token_stream(&ast, "left");
    let top = get_list_attribute_token_stream(&ast, "top");

    let generics = ast.generics;
    let mut generics_mut = generics.clone();

    let where_clause = generics_mut.make_where_clause();

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let struct_name = ast.ident;

    TokenStream::from(quote! {
        impl #impl_generics nice_bevy_utils::layout::layout_positioning::LayoutPositioning for #struct_name #ty_generics #where_clause
        {
            type Context<'a> = ();

            fn size(&self, _context: &Self::Context<'_>, _sizing: &nice_bevy_utils::layout::layout_sizing::LayoutSizing) -> bevy::prelude::Vec2 {
                let x = #width;
                let y = #height;
                Vec2{x,y}
            }

            fn location(&self, _context: &Self::Context<'_>, _sizing: &nice_bevy_utils::layout::layout_sizing::LayoutSizing) -> bevy::prelude::Vec2 {
                let x = #left;
                let y = #top;
                Vec2{x,y}
            }
        }
    })
}

#[proc_macro_derive(HasFontSize, attributes(font_size))]
pub fn derive_has_font_size(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let font_size_expression = get_list_attribute_token_stream(&ast, "font_size");

    let generics = ast.generics;
    let mut generics_mut = generics.clone();

    let where_clause = generics_mut.make_where_clause();

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let struct_name = ast.ident;

    TokenStream::from(quote! {
        impl #impl_generics nice_bevy_utils::layout::layout_positioning::HasFontSize for #struct_name #ty_generics #where_clause
        {
            type FontContext<'a> = ();

            fn font_size(&self, context: &Self::FontContext<'_>) -> f32 {
                #font_size_expression
            }
        }
    })
}

#[proc_macro_derive(HasOrigin, attributes(origin))]
pub fn derive_has_origin(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let origin_expression = get_list_attribute_token_stream(&ast, "origin");

    let generics = ast.generics;
    let mut generics_mut = generics.clone();

    let where_clause = generics_mut.make_where_clause();

    let (impl_generics, ty_generics, _where_clause) = generics.split_for_impl();

    let struct_name = ast.ident;

    TokenStream::from(quote! {
        impl #impl_generics nice_bevy_utils::layout::layout_positioning::HasOrigin for #struct_name #ty_generics #where_clause
        {
            fn origin(&self, context: &Self::Context<'_>, sizing: &nice_bevy_utils::layout::layout_sizing::LayoutSizing) -> nice_bevy_utils::layout::layout_positioning::Origin
            {
                 #origin_expression
            }
        }
    })
}


