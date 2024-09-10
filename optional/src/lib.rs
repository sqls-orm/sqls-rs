extern crate proc_macro;
extern crate quote;
extern crate syn;

#[proc_macro_derive(Optional)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let fields = match &input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    let opt_fields = fields.named.iter().map(|field| {
                        let attr_name = &field.ident;
                        let attr_ty = &field.ty;
                        quote::quote! {
                            #[sqlx(default)]
                            pub #attr_name: Option<#attr_ty>
                        }
                    });
                    quote::quote! { { #(#opt_fields,)* } }
                }
                _ => unimplemented!("#[derive(Optional)] supports only named struct attributes"),
            }
        }
        _ => unimplemented!("#[derive(Optional)] supports only structs"),
    };

    let columns = match &input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap().to_string())
                        .collect::<Vec<_>>()
                }
                _ => unimplemented!("#[derive(Optional)] supports only named struct attributes"),
            }
        }
        _ => unimplemented!("#[derive(Optional)] supports only structs"),
    };

    let sct_setters = match &input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    let setters = fields.named.iter().map(|field| {
                        let attr_name = &field.ident;
                        match &field.ty {
                            syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("String") => {
                                quote::quote! { #attr_name: self.#attr_name.as_deref().unwrap().to_string() }
                            }
                            _ => quote::quote! { #attr_name: self.#attr_name.unwrap() }
                        }
                    });
                    quote::quote! { { #(#setters,)* } }
                }
                _ => unimplemented!("#[derive(Optional)] supports only named struct attributes"),
            }
        }
        _ => unimplemented!("#[derive(Optional)] supports only structs"),
    };

    let mdl_setters = match input.data {
        syn::Data::Struct(data) => {
            match data.fields {
                syn::Fields::Named(ref fields) => {
                    let setters = fields.named.iter().map(|field| {
                        let attr_name = &field.ident;
                        quote::quote! { #attr_name: Some(self.#attr_name) }
                    });
                    quote::quote! { { #(#setters,)* } }
                }
                _ => unimplemented!("#[derive(Optional)] supports only named struct attributes"),
            }
        }
        _ => unimplemented!("#[derive(Optional)] supports only structs"),
    };

    let sct_ident = input.ident;
    let mdl_ident = syn::Ident::new(&format!("{}Optional", sct_ident), sct_ident.span());

    proc_macro::TokenStream::from(quote::quote! {
        #[cfg_attr(feature = "debug", derive(Debug))]
        #[cfg_attr(feature = "clone", derive(Clone))]
        #[cfg_attr(feature = "copy", derive(Copy))]
        #[derive(sqlx::FromRow)]
        pub struct #mdl_ident #fields

        impl #sct_ident {
            pub fn optional(self) -> #mdl_ident {
                #mdl_ident #mdl_setters
            }
        }

        impl #mdl_ident {
            #[cfg(feature = "gql")]
            pub async fn columns() -> &'static Vec<std::string::String> {
                use tokio::sync::OnceCell;

                static VEC: OnceCell<Vec<std::string::String>> = OnceCell::const_new();
                VEC.get_or_init(|| async { vec![
                    #(#columns.to_string()),*
                ] }).await
            }

            #[cfg(feature = "gql")]
            pub async fn extract<'ctx>(
                ctx: Option<&async_graphql::Context<'ctx>>,
            ) -> &'static str {
                match ctx {
                    Some(context) => {
                        use convert_case::Casing;
                        use convert_case as convert;

                        let columns = #mdl_ident::columns().await;

                        ctx
                            .field()
                            .selection_set()
                            .filter_map(|field| {
                                let name: std::string::String = field.name().to_case(convert::Case::Snake);
                                match columns.contains(&name) {
                                    true => Some(name),
                                    false => None,
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                            .as_str()
                    },
                    None => "*"
                }
            }

            pub fn original(self) -> #sct_ident {
                #sct_ident #sct_setters
            }
        }
    })
}
