use darling::{
    ast::{Data, Fields},
    util, FromDeriveInput, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[allow(unused)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(error_info))]
struct ErrorData {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<EnumVariants, ()>,

    app_type: syn::Type,
    prefix: String,
}

#[allow(unused)]
#[derive(Debug, FromVariant)]
#[darling(attributes(error_info))]
struct EnumVariants {
    ident: syn::Ident,
    fields: Fields<util::Ignored>,

    code: String,
    #[darling(default)]
    app_code: String,
    #[darling(default)]
    client_msg: String,
}

pub(crate) fn process_to_error_info(input: DeriveInput) -> TokenStream {
    let ErrorData {
        ident: name,
        generics,
        data: Data::Enum(data),
        app_type,
        prefix,
    } = ErrorData::from_derive_input(&input).expect("Can not parse input as ErrorData")
    else {
        panic!("Only enums are supported as input. Got: {}", input.ident);
    };

    let code = data
        .iter()
        .map(|v| {
            let EnumVariants {
                ident,
                code,
                app_code,
                client_msg,
                fields: _,
            } = v;
            let code = format!("{}{}", prefix, code);
            quote! {
                #name::#ident(_) => {
                    ErrorInfo::try_new(
                        #app_code,
                        #code,
                        #client_msg,
                        self,
                    )
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        use error_code::{ErrorInfo, ToErrorInfo};
        impl #generics ToErrorInfo for #name #generics {
            type T = #app_type;
            fn to_error_info(&self) -> Result<ErrorInfo<Self::T>,<Self::T as std::str::FromStr>::Err> {
                match self {
                    #(#code),*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_to_error_info() {
        let input = r#"
            #[derive(Debug,ToErrorInfo)]
            #[error_info(app_type = "http::StatusCode", prefix = "api_")]
            pub enum MyError {
                #[error("Invalid command: {0}")]
                #[error_info(code = "IC", app_code = "201")]
                InvalidCommand(String),
                #[error("Invalid command: {0}")]
                #[error_info(code = "IA", app_code = "202", client_msg = "friendly msg")]
                InvalidArgument(String),

                #[error("Request timeout")]
                #[error_info(code = "RE", app_code = "500")]
                RespError(#[from] std::io::Error),
            }
        "#;
        let ast = syn::parse_str(input).unwrap();
        let error_data = ErrorData::from_derive_input(&ast).unwrap();

        dbg!("{:#?}", error_data);

        let code = process_to_error_info(ast);

        println!("{}", code);
    }
}
