use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PartialBorrow, attributes(borrow_id))]
pub fn borrow_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data
        && let syn::DataStruct { fields, .. } = data
        && let syn::Fields::Named(fields) = fields
    {
        &fields.named
    } else {
        panic!("PartialBorrow only supports structs with named fields.");
    };

    let (peq, hash, borrow, types) = fields
        .iter()
        .flat_map(|f| {
            f.attrs.iter().flat_map(|a| {
                a.meta
                    .require_path_only()
                    .is_ok_and(|f| f.is_ident("borrow_id"))
                    .then(|| {
                        let ty = f.ty.clone();
                        let id = f.ident.as_ref().unwrap(); // we already checked that struct have
                                                            // only named fields so unwrap is ok

                        (
                            quote! {self.#id == other.#id},
                            quote! {self.#id.hash(state);},
                            quote! {self.#id},
                            quote! {#ty},
                        )
                    })
            })
        })
        .collect::<(Vec<_>, Vec<_>, Vec<_>, Vec<_>)>();

    if peq.len() != 1 {
        // HACK: couldn't manage to borrow multiple fields without extra trait and extra struct, so will assure only one borrow field.
        panic!("Add exectly one `borrow_id` attribute.");
    };

    quote! {
        impl ::core::cmp::PartialEq for #ident {
            fn eq(&self, other: &#ident) -> bool {
                #(#peq)&&*
            }
        }

        impl ::core::cmp::Eq for #ident {}

        impl ::std::hash::Hash for #ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                #(#hash)*
            }
        }

        impl ::std::borrow::Borrow<(#(#types),*)> for #ident {
            fn borrow(&self) -> &(#(#types),*) {
                &( #(#borrow),* )
            }
        }
    }
    .into()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
