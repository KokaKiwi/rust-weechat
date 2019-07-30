#![recursion_limit="256"]

extern crate proc_macro;
use proc_macro2::{Ident, Literal};
use std::collections::HashMap;

use syn::{parse_macro_input, Error, LitStr};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

use quote::quote;


struct WeechatPluginInfo {
    plugin: syn::Ident,
    name: (usize, Literal),
    author: (usize, Literal),
    description: (usize, Literal),
    version: (usize, Literal),
    license: (usize, Literal),
}

enum WeechatVariable {
    Name(syn::LitStr),
    Author(syn::LitStr),
    Description(syn::LitStr),
    Version(syn::LitStr),
    License(syn::LitStr),
}

impl WeechatVariable {
    fn litstr_to_pair(string: &LitStr) -> (usize, Literal) {
        let mut bytes = string.value().into_bytes();
        bytes.push(0);
        let len = bytes.len();
        (len, Literal::byte_string(&bytes))
    }

    fn as_pair(&self) -> (usize, Literal) {
        match self {
            WeechatVariable::Name(string) => WeechatVariable::litstr_to_pair(string),
            WeechatVariable::Author(string) => WeechatVariable::litstr_to_pair(string),
            WeechatVariable::Description(string) => WeechatVariable::litstr_to_pair(string),
            WeechatVariable::Version(string) => WeechatVariable::litstr_to_pair(string),
            WeechatVariable::License(string) => WeechatVariable::litstr_to_pair(string),
        }
    }
}

impl Parse for WeechatVariable {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let value = input.parse()?;

        match key.to_string().to_lowercase().as_ref() {
            "name" => Ok(WeechatVariable::Name(value)),
            "author" => Ok(WeechatVariable::Author(value)),
            "description" => Ok(WeechatVariable::Description(value)),
            "version" => Ok(WeechatVariable::Version(value)),
            "license" => Ok(WeechatVariable::License(value)),
            _ => Err(Error::new(key.span(), "expected one of bla"))
        }
    }
}


impl Parse for WeechatPluginInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let plugin: syn::Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        let args: Punctuated::<WeechatVariable, syn::Token![,]> = input.parse_terminated(WeechatVariable::parse)?;
        let mut variables = HashMap::new();

        for arg in args.pairs() {
            let variable = arg.value();
            match variable {
                WeechatVariable::Name(_) => variables.insert("name", *variable),
                WeechatVariable::Author(_) => variables.insert("author", *variable),
                WeechatVariable::Description(_) => variables.insert("description", *variable),
                WeechatVariable::Version(_) => variables.insert("version", *variable),
                WeechatVariable::License(_) => variables.insert("license", *variable),
            };
        }

        Ok(WeechatPluginInfo {
            plugin,
            name: variables.remove("name").unwrap().as_pair(),
            author: variables.remove("author").unwrap().as_pair(),
            description: variables.remove("description").unwrap().as_pair(),
            version: variables.remove("version").unwrap().as_pair(),
            license: variables.remove("license").unwrap().as_pair(),
        })
    }
}

#[proc_macro]
pub fn weechat_plugin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WeechatPluginInfo {
        plugin,
        name,
        author,
        description,
        version,
        license
    } = parse_macro_input!(input as WeechatPluginInfo);

    let (name_len, name) = name;
    let (author_len, author) = author;
    let (description_len, description) = description;
    let (license_len, license) = license;
    let (version_len, version) = version;

    let result = quote! {
        #[no_mangle]
        pub static weechat_plugin_api_version: [u8; weechat_sys::WEECHAT_PLUGIN_API_VERSION_LENGTH] = *weechat_sys::WEECHAT_PLUGIN_API_VERSION;

        #[no_mangle]
        pub static weechat_plugin_name: [u8; #name_len] = *#name;

        #[no_mangle]
        pub static weechat_plugin_author: [u8; #author_len] = *#author;

        #[no_mangle]
        pub static weechat_plugin_description: [u8; #description_len] = *#description;

        #[no_mangle]
        pub static weechat_plugin_version: [u8; #version_len] = *#version;

        #[no_mangle]
        pub static weechat_plugin_license: [u8; #license_len] = *#license;

        static mut __PLUGIN: Option<#plugin> = None;

        #[no_mangle]
        pub extern "C" fn weechat_plugin_init(
            plugin: *mut weechat_sys::t_weechat_plugin,
            argc: libc::c_int,
            argv: *mut *mut ::libc::c_char,
        ) -> libc::c_int {
            let plugin = Weechat::from_ptr(plugin);
            let args = ArgsWeechat::new(argc, argv);
            match <#plugin as ::weechat::WeechatPlugin>::init(plugin, args) {
                Ok(p) => {
                    unsafe { __PLUGIN = Some(p) }
                    return weechat_sys::WEECHAT_RC_OK;
                }
                Err(_e) => {
                    return weechat_sys::WEECHAT_RC_ERROR;
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn weechat_plugin_end(_plugin: *mut weechat_sys::t_weechat_plugin) -> ::libc::c_int {
            unsafe {
                __PLUGIN = None;
            }
            weechat_sys::WEECHAT_RC_OK
        }
    };

    result.into()
}
