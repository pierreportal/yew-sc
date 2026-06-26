mod utils;

use proc_macro::TokenStream;
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};
use utils::parser::{CssBlock, hash_css};
use utils::tags::{is_void_tag, validate_tag};

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
        validate_tag(&tag)?;
        let css: CssBlock = input.parse()?;
        Ok(Self { name, tag, css })
    }
}

const PLACEHOLDER: &str = "__YEW_SC_SELF__";

fn build_css(css: &CssBlock) -> (String, String) {
    let rules = css.to_rules(PLACEHOLDER);
    let placeholder_css = rules
        .iter()
        .map(|(sel, body)| format!("{} {{ {} }}", sel, body))
        .collect::<Vec<_>>()
        .join(" ");
    let class_name = hash_css(&placeholder_css);
    let selector = format!(".{}", class_name);
    let full_css = placeholder_css.replace(PLACEHOLDER, &selector);
    (class_name, full_css)
}

fn codegen_component(
    component_name: &syn::Ident,
    tag: &syn::Ident,
    class_name: String,
    css_string: String,
) -> TokenStream {
    let expended = quote::quote! {
        #[::yew::component]
        pub fn #component_name(props: &::yew_sc::StyledComponentProps) -> ::yew::Html {
            ::yew::use_effect(|| {
                ::yew_sc::register_style(#class_name, #css_string)
            });
            ::yew::html! {
                <#tag
                    class={::yew::classes!(#class_name, props.class.clone())}
                    onclick={props.onclick.clone()}
                    id={props.id.clone()}
                    title={props.title.clone()}
                    hidden={props.hidden}
                    tabindex={props.tabindex.clone()}
                    role={props.role.clone()}
                >
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
        #[::yew::component]
        pub fn #component_name(props: &::yew_sc::StyledVoidComponentProps) -> ::yew::Html {
            ::yew::use_effect(|| {
                ::yew_sc::register_style(#class_name, #css_string)
            });
            ::yew::html! {
                <#tag
                    class={::yew::classes!(#class_name, props.class.clone())}
                    onclick={props.onclick.clone()}
                    id={props.id.clone()}
                    title={props.title.clone()}
                    hidden={props.hidden}
                    tabindex={props.tabindex.clone()}
                    role={props.role.clone()}
                />
            }
        }
    };

    expended.into()
}

#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentInput);
    let (class_name, css_string) = build_css(&input.css);
    let component_name = &input.name;
    let tag = &input.tag;

    if is_void_tag(&tag.to_string()) {
        codegen_void_component(component_name, tag, class_name, css_string)
    } else {
        codegen_component(component_name, tag, class_name, css_string)
    }
}
