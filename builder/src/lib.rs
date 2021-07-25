use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Visibility};



#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);

    let DeriveInput {
        vis, ident, data, ..
    } = parsed;

    let fields = get_fields(data);

    let builder_struct_ident = get_builder_struct_ident(ident.clone());
    let builder_struct = create_builder_struct(vis.clone(), builder_struct_ident.clone(), fields.clone());

    let field_nones = fields.into_iter().map(|Field { ident, .. }| {
        quote! { #ident: None }
    });

    let expanded = quote! {
        #builder_struct

        impl #ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #(#field_nones),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_fields(data: Data) -> Vec<Field> {
    if let Data::Struct(struct_data) = data {
        if let Fields::Named(fields) = struct_data.fields {
            return fields.named.into_iter().collect();
        } else {
            panic!("#[derive(Builder)] only works with named struct fields");
        }
    } else {
        panic!("#[derive(Builder)] only works with structs");
    }
}

fn get_builder_struct_ident(ident: Ident) -> Ident {
    format_ident!("{}Builder", ident)
}

fn create_builder_struct(
    vis: Visibility,
    ident: Ident,
    fields: Vec<Field>,
) -> proc_macro2::TokenStream {
    let setters = create_setters(fields.clone());

    let new_fields = fields.iter().map(|Field { ident, ty, .. }| {
        quote! {
           #ident: Option<#ty>
        }
    });

    return quote! {
        #vis struct #ident {
            #(#new_fields),*
        }

        impl #ident {
            #(#setters)*
        }
    };
}

fn create_setters(fields: Vec<Field>) -> Vec<proc_macro2::TokenStream> {
    fields
        .iter()
        .map(|Field { ident, ty, .. }| {
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        })
        .collect()
}
