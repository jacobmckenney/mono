extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasTimestamps)]
pub fn derive_has_timestamps(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the type we're deriving the trait for
    let name = input.ident;

    // Generate the implementation of the HasTimestamps trait
    let expanded = quote! {
        impl HasTimestamps for #name {
            fn created_at(&self) -> sea_orm::ActiveValue<chrono::NaiveDateTime> {
                self.created_at.clone()
            }

            fn updated_at(&self) -> sea_orm::ActiveValue<chrono::NaiveDateTime> {
                self.updated_at.clone()
            }

            fn set_created_at(&mut self, created_at: chrono::NaiveDateTime) {
                self.created_at = sea_orm::ActiveValue::Set(created_at);
            }

            fn set_updated_at(&mut self, updated_at: chrono::NaiveDateTime) {
                self.updated_at = sea_orm::ActiveValue::Set(updated_at);
            }
        }
    };

    // Convert the expanded code into a TokenStream and return it
    TokenStream::from(expanded)
}
