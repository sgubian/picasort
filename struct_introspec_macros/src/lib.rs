//! Author: Sylvain Gubian <sgubian@lemur-catta.org>

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, PathArguments, Type};

#[proc_macro_derive(DynamicGetSet)]
pub fn dynamic_getset_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => panic!("DynamicGetSet can only be used with structs"),
    };

    // Generate match arms for `set_field_by_index`
    let set_index_match_arms = fields.iter().enumerate().filter_map(|(index, field)| {
        let field_name = field.ident.as_ref()?;
        let field_ty = &field.ty;

        Some(quote! {
            #index => {
                if let Ok(value) = value.downcast::<#field_ty>() {
                    self.#field_name = *value;
                    Ok(())
                } else {
                    Err("Type mismatch for field")
                }
            }
        })
    });

    // Generate match arms for `set_field_by_name`
    let set_name_match_arms = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();
        let field_ty = &field.ty;

        Some(quote! {
            #field_name_str => {
                if let Ok(value) = value.downcast::<#field_ty>() {
                    self.#field_name = *value;
                    Ok(())
                } else {
                    Err("Type mismatch for field")
                }
            }
        })
    });

    // Generate match arms for `get_value_by_field_name`
    let get_name_match_arms = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();
        let field_ty = &field.ty;

        // Detect if the type is Option<T>
        let is_option = match field_ty {
            Type::Path(type_path) if type_path.qself.is_none() => {
                type_path.path.segments.last().is_some_and(|seg| {
                    seg.ident == "Option"
                        && matches!(seg.arguments, PathArguments::AngleBracketed(_))
                })
            }
            _ => false,
        };

        if is_option {
            // Special handling: return None if Option<T> is None
            Some(quote! {
                #field_name_str => {
                    match &self.#field_name {
                        Some(inner) => Some(inner as &dyn std::any::Any),
                        None => None,
                    }
                }
            })
        } else {
            // Normal field
            Some(quote! {
                #field_name_str => Some(&self.#field_name as &dyn std::any::Any),
            })
        }
    });

    // Generate field names as a vector
    let field_names = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_name_str = field_name.to_string();

        Some(quote! {
            #field_name_str
        })
    });

    let expanded = quote! {
        impl DynamicGetSet for #struct_name {
            fn set_field_by_index(&mut self, index: usize, value: Box<dyn std::any::Any>) -> Result<(), &'static str> {
                match index {
                    #(#set_index_match_arms),*
                    _ => Err("Invalid index"),
                }
            }

            fn set_field_by_name(&mut self, name: &str, value: Box<dyn std::any::Any>) -> Result<(), &'static str> {
                match name {
                    #(#set_name_match_arms),*
                    _ => Err("Invalid field name"),
                }
            }

            fn get_field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }

            fn get_value_by_field_name(&self, name: &str) -> Option<&dyn std::any::Any> {
                match name {
                    #(#get_name_match_arms)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
