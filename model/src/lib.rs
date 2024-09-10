extern crate proc_macro;
extern crate quote;
extern crate syn;
extern crate builder;

#[proc_macro_derive(Model)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let columns = match &input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    let _columns = fields.named.iter().map(|field| {
                        let attr_name = &field.ident;
                        let name = attr_name.as_ref().unwrap().to_string();

                        quote::quote! {
                            pub fn #attr_name() -> builder::Column {
                                builder::Column::new(#name)
                            }
                        }
                    });
                    quote::quote! { { #(#_columns)* } }
                }
                _ => unimplemented!("#[derive(Model)] supports only named struct attributes"),
            }
        }
        _ => unimplemented!("#[derive(Model)] supports only structs"),
    };

    let sct_ident = input.ident;
    let tablename = sct_ident.to_string();

    proc_macro::TokenStream::from(quote::quote! {
        impl #sct_ident #columns

        impl builder::Model for #sct_ident {
            fn table() -> &'static str {
                #tablename
            }
        }
    })
}
