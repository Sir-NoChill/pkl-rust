use syn::{parse_macro_input, DeriveInput, Data, Fields};
use quote::quote;

#[proc_macro_derive(Pkl)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let (tuple_types, setters) = depkl_types(&input.data);

    // Need to decompose the struct into the pkl-associated fields
    // in a tuple.
    let res = quote! {
        use rmp_serde;

        impl Pkl for #ident {
            fn unmarshal(data: Vec<u8>) -> Result<#ident, &'static str> {
                let decoded: (i64, ::std::string::String, ::std::string::String, #tuple_types)
                    = rmp_serde::decode::from_slice(&data).expect("Failed to decode");

                let values = decoded.3;  // TODO extract the name here to a variable

                let ret = #ident { #setters };
                return Ok(ret);
            }
        }
    };

    return res.into();
}

fn depkl_types(input: &Data) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match *input {
        Data::Struct(ref input) => {
            match input.fields {
                Fields::Named(ref fields) => {
                    // Expands to Option<field> for each field in the struct
                    //
                    // Using the fully qualified syntax
                    let recursed = fields.named.iter().map(|f| {
                        let ty = &f.ty;

                        quote! {
                            (i64, ::std::string::String, #ty)
                        }});

                    let types = quote! {
                        (#(#recursed),*)
                    };

                    let mut setters_recursed: Vec<proc_macro2::TokenStream> = vec![];

                    let mut i = 0;
                    for field in fields.named.iter() {
                        let ident = &field.ident;
                        let ind = syn::Index::from(i);
                        let setter = quote! {
                            #ident: values.#ind.2.into()
                        };

                        setters_recursed.push(setter);
                        i = i + 1;
                    };

                    let setters = quote! {
                        #(#setters_recursed),*
                    };

                    return (types, setters);
                }
                _ => unimplemented!(),
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
