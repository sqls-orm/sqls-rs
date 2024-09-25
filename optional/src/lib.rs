use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta};
use syn::spanned::Spanned;

fn gen_model_attributes(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let default = match cfg!(feature = "sqlx") { // TODO: push this into `attributes`
        true => quote!(
            #[sqlx(default)]
        ),
        false => quote! {},
    };
    let attrs = fields.iter().map(|field| {
        let attributes = &field.attrs;
        let name = &field.ident;
        let ty = &field.ty;
        quote!(
            #default
            #(#attributes)*
            pub #name: Option<#ty>
        )
    });
    quote!(
        { #(#attrs,)* }
    )
}

fn parse_derive_features() -> TokenStream {
    let mut derives: Vec<TokenStream> = Vec::default();
    if cfg!(feature = "debug") {
        derives.push(quote!(Debug));
    }
    if cfg!(feature = "clone") {
        derives.push(quote!(Clone));
    }
    if cfg!(feature = "copy") {
        derives.push(quote!(Copy));
    }
    if cfg!(feature = "sqlx") {
        derives.push(quote!(sqlx::FromRow));
    }
    quote! {
        #[derive( #(#derives),* )]
    }
}

fn extract_derive_attribute(input: &DeriveInput, attribute: &str) -> Option<String> {
    for attr in input.attrs.iter() {
        if attr.path().get_ident().unwrap().to_string().as_str() == "sql" {
            let list = attr.meta.require_list().unwrap();
            let meta: syn::MetaNameValue = syn::parse2(list.tokens.clone()).unwrap();
            if meta.path.get_ident().unwrap().to_string().as_str().eq(attribute) {
                return Some(meta.value.into_token_stream().to_string().replace('"', ""));
            }
        }
    }
    return None;
}

fn parse_derive_attributes(input: &DeriveInput) -> (String, char) {
    let table = extract_derive_attribute(input, "table")
        .unwrap_or_else(|| convert_case::Casing::to_case(&input.ident.to_string(), convert_case::Case::Snake));
    let wrapper = extract_derive_attribute(input, "wrapper")
        .unwrap_or_else(|| String::from("\""));
    (table, wrapper.chars().nth(0).unwrap())
}

fn parse_column_names(
    fields: &Punctuated<Field, Comma>
) -> Vec<String> {
    fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>()
}

fn expand_struct_constructor(
    fields: &Punctuated<Field, Comma>
) -> TokenStream {
    let setters = fields.iter().map(|field| {
        let name = &field.ident;
        match &field.ty {
            syn::Type::Path(syn::TypePath { path, .. }) if path.is_ident("String") => {
                quote::quote!(
                    #name: self.#name.as_deref().unwrap().to_string()
                )
            }
            _ => quote::quote!(
                #name: self.#name.unwrap()
            )
        }
    });
    quote::quote!(
        { #(#setters,)* }
    )
}

fn expand_model_constructor(
    fields: &Punctuated<Field, Comma>
) -> TokenStream {
    let setters = fields.iter().map(|field| {
        let name = &field.ident;
        quote::quote!(
            #name: std::option::Option::Some(self.#name)
        )
    });
    quote::quote!(
        { #(#setters,)* }
    )
}

fn expand_struct_to_model(
    model_id: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let constructor = expand_model_constructor(fields);

    quote!(
        pub fn model(self) -> #model_id {
            #model_id #constructor
        }
    )
}

fn expand_model_to_struct(
    struct_id: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let constructor = expand_struct_constructor(fields);

    quote!(
        pub fn structure(self) -> #struct_id {
            #struct_id #constructor
        }
    )
}

fn expand_model_columns(
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let columns = parse_column_names(fields);

    quote!(
        pub async fn columns() -> &'static std::vec::Vec<std::string::String> {
            use tokio::sync::OnceCell;

            static VEC: OnceCell<std::vec::Vec<std::string::String>> = OnceCell::const_new();
            VEC.get_or_init(|| async { vec![
                #(#columns.to_string()),*
            ] }).await
        }
    )
}

fn expand_model_columnify(
    input: &DeriveInput,
) -> TokenStream {
    let (table, wrapper) = parse_derive_attributes(input);

    quote!(
        pub fn columnify(name: impl std::fmt::Display) -> std::string::String {
            return std::format!("{w}{}{w}.{w}{}{w}", #table, name, w=#wrapper);
        }
    )
}

fn expand_model_extract(
    input: &DeriveInput,
    model_id: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    if !cfg!(feature = "gql") {
        return quote!();
    }

    let columns = expand_model_columns(fields);
    let columnify = expand_model_columnify(input);

    quote!(
        #columns
        #columnify

        pub async fn extract<'ctx>(
            ctx: std::option::Option<&async_graphql::Context<'ctx>>,
        ) -> std::string::String {
            match ctx {
                std::option::Option::Some(context) => {
                    let columns = #model_id::columns().await;

                    let mut selection = context
                        .field()
                        .selection_set()
                        .filter_map(|field| {
                            let name: std::string::String = convert_case::Casing::to_case(&field.name(), convert_case::Case::Snake);
                            columns.contains(&name).then(|| #model_id::columnify(name))
                        })
                        .collect::<std::vec::Vec<std::string::String>>();
                    let primary = #model_id::columnify("id");
                    (!selection.contains(&primary)).then(|| selection.push(primary));

                    selection.join(", ")
                },
                std::option::Option::None => #model_id::columnify("*")
            }
        }
    )
}

fn expand_derive_model_struct_named(
    input: &DeriveInput,
    fields: &Punctuated<Field, Comma>,
) -> syn::Result<TokenStream> {
    let derive = parse_derive_features();

    let struct_id = &input.ident;
    let model_id = Ident::new(&format!("{}Model", struct_id), struct_id.span());

    let model_attributes = gen_model_attributes(fields);

    let to_model = expand_struct_to_model(&model_id, fields);
    let to_struct = expand_model_to_struct(&struct_id, fields);
    let extract = expand_model_extract(input, &model_id, fields);

    Ok(quote!(
        #derive
        pub struct #model_id
        #model_attributes

        impl #struct_id {
            #to_model
        }

        impl #model_id {
            #to_struct

            #extract
        }
    ))
}

fn expand_derive_model(
    input: &DeriveInput
) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(
            DataStruct {
                fields: Fields::Named(FieldsNamed { named, .. }),
                ..
            }
        ) => expand_derive_model_struct_named(input, named),

        Data::Struct(
            DataStruct {
                fields: Fields::Unnamed(_),
                ..
            }
        ) => Err(syn::Error::new_spanned(input, "unnamed fields are not supported")),

        Data::Struct(
            DataStruct {
                fields: Fields::Unit,
                ..
            }
        ) => Err(syn::Error::new_spanned(input, "unit structs are not supported")),

        Data::Enum(_) => Err(syn::Error::new_spanned(input, "enums are not supported")),

        Data::Union(_) => Err(syn::Error::new_spanned(input, "unions are not supported")),
    }
}

#[proc_macro_derive(Model, attributes(sql))]
pub fn derive_model(
    input: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    match expand_derive_model(&input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}