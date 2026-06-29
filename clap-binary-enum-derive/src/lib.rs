use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(YesNoArg, attributes(yesno))]
pub fn derive_yes_no_arg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_yes_no_arg_inner(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn derive_yes_no_arg_inner(input: DeriveInput) -> syn::Result<TokenStream2> {
    let enum_name = &input.ident;

    let enum_attr = parse_yesno_enum_attrs(&input.attrs)?;

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => {
            return Err(syn::Error::new(
                enum_name.span(),
                "YesNoArg can only be used on enums",
            ))
        }
    };

    let variant_vec: Vec<_> = variants.iter().collect();
    if variant_vec.len() != 2 {
        return Err(syn::Error::new(
            enum_name.span(),
            "YesNoArg requires exactly two variants",
        ));
    }

    for v in &variant_vec {
        if !matches!(v.fields, Fields::Unit) {
            return Err(syn::Error::new(
                v.ident.span(),
                "YesNoArg variants must have no fields",
            ));
        }
    }

    let pos_variant = &variant_vec[0];
    let neg_variant = &variant_vec[1];
    let pos = &pos_variant.ident;
    let neg = &neg_variant.ident;

    let pos_attr = parse_yesno_variant_attrs(&pos_variant.attrs)?;
    let neg_attr = parse_yesno_variant_attrs(&neg_variant.attrs)?;

    let arg_struct_name = syn::Ident::new(&format!("{}Arg", enum_name), enum_name.span());

    let stem = enum_attr
        .name
        .unwrap_or_else(|| to_kebab_case(&enum_name.to_string()));
    let pos_flag_name = stem.clone();
    let neg_flag_name = format!("no-{}", stem);

    let pos_field = syn::Ident::new(&stem.replace('-', "_"), enum_name.span());
    let neg_field = syn::Ident::new(&format!("no_{}", stem.replace('-', "_")), enum_name.span());

    let pos_help = pos_attr
        .help
        .as_ref()
        .map(|h| quote! { , help = #h });
    let neg_help = neg_attr
        .help
        .as_ref()
        .map(|h| quote! { , help = #h });

    Ok(quote! {
        #[derive(clap::Args, Debug, Clone)]
        #[group(multiple = false)]
        pub struct #arg_struct_name {
            #[arg(long = #pos_flag_name #pos_help)]
            #pos_field: bool,

            #[arg(long = #neg_flag_name #neg_help)]
            #neg_field: bool,
        }

        impl #arg_struct_name {
            pub fn get(&self) -> Option<#enum_name> {
                match (self.#pos_field, self.#neg_field) {
                    (true, _) => Some(#enum_name::#pos),
                    (_, true) => Some(#enum_name::#neg),
                    _         => None,
                }
            }
        }

        impl From<#arg_struct_name> for Option<#enum_name> {
            fn from(arg: #arg_struct_name) -> Self {
                arg.get()
            }
        }
    })
}

struct YesnoEnumAttr {
    name: Option<String>,
}

struct YesnoVariantAttr {
    help: Option<String>,
}

fn parse_yesno_enum_attrs(attrs: &[syn::Attribute]) -> syn::Result<YesnoEnumAttr> {
    let mut name = None;
    for attr in attrs {
        if attr.path().is_ident("yesno") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    name = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unexpected yesno attribute; expected `name`"))
                }
            })?;
        }
    }
    Ok(YesnoEnumAttr { name })
}

fn parse_yesno_variant_attrs(attrs: &[syn::Attribute]) -> syn::Result<YesnoVariantAttr> {
    let mut help = None;
    for attr in attrs {
        if attr.path().is_ident("yesno") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("help") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    help = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unexpected yesno attribute; expected `help`"))
                }
            })?;
        }
    }
    Ok(YesnoVariantAttr { help })
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('-');
        }
        result.push(ch.to_lowercase().next().unwrap());
    }
    result
}
