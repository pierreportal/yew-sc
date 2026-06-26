mod utils;

use proc_macro::TokenStream;
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};
use utils::parser::{CssBlock, hash_css};

struct StyledComponentInput {
    pub name: syn::Ident,
    pub tag: syn::Ident,
    pub css: CssBlock,
}

impl Parse for StyledComponentInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let tag: syn::Ident = input.parse()?;
        let css: CssBlock = input.parse()?;
        Ok(Self { name, tag, css })
    }
}

fn codegen_component(
    component_name: &syn::Ident,
    tag: &syn::Ident,
    class_name: String,
    css_string: String,
) -> TokenStream {
    let expended = quote::quote! {
        #[yew::component]
        pub fn #component_name(props: &StyledComponentProps) -> yew::Html {
            yew::use_effect(|| {
                register_style(#class_name, #css_string)
            });
            yew::html! {
                <#tag onclick={props.onclick.clone()} class={classes!(#class_name, props.class.clone())}>
                    {for props.children.iter()}
                </#tag>
            }
        }
    };

    expended.into()
}
fn codegen_void_component(
    component_name: &syn::Ident,
    tag: &syn::Ident,
    class_name: String,
    css_string: String,
) -> TokenStream {
    let expended = quote::quote! {
        #[yew::component]
        pub fn #component_name(props: &StyledVoidComponentProps) -> yew::Html {
            yew::use_effect(|| {
                register_style(#class_name, #css_string)
            });
            yew::html! {
                <#tag class={classes!(#class_name, props.class.clone())}/>
            }
        }
    };

    expended.into()
}

#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentInput);
    let css_string = input.css.to_css();
    let class_name = hash_css(&css_string);
    let component_name = &input.name;
    let tag = &input.tag;

    let is_void = matches!(
        tag.to_string().as_str(),
        "input" | "img" | "br" | "hr" | "meta" | "link"
    );

    if is_void {
        codegen_void_component(component_name, tag, class_name, css_string)
    } else {
        codegen_component(component_name, tag, class_name, css_string)
    }
}
