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

fn codegen(
    component_name: &syn::Ident,
    tag: &syn::Ident,
    class_name: String,
    css_string: String,
    props_ident: proc_macro2::TokenStream,
    extra_attrs: proc_macro2::TokenStream,
    is_void: bool,
) -> TokenStream {
    let body = if is_void {
        quote::quote! {
            <#tag
                class={::yew::classes!(#class_name, props.class.clone())}
                onclick={props.onclick.clone()}
                id={props.id.clone()}
                title={props.title.clone()}
                hidden={props.hidden}
                tabindex={props.tabindex.clone()}
                role={props.role.clone()}
                #extra_attrs
            />
        }
    } else {
        quote::quote! {
            <#tag
                class={::yew::classes!(#class_name, props.class.clone())}
                onclick={props.onclick.clone()}
                id={props.id.clone()}
                title={props.title.clone()}
                hidden={props.hidden}
                tabindex={props.tabindex.clone()}
                role={props.role.clone()}
                #extra_attrs
            >
                {for props.children.iter()}
            </#tag>
        }
    };

    let expanded = quote::quote! {
        #[::yew::component]
        pub fn #component_name(props: &::yew_sc::#props_ident) -> ::yew::Html {
            ::yew::use_effect(|| {
                ::yew_sc::register_style(#class_name, #css_string)
            });
            ::yew::html! { #body }
        }
    };

    expanded.into()
}

fn props_for_tag(tag: &str) -> (proc_macro2::TokenStream, proc_macro2::TokenStream, bool) {
    use quote::quote;
    match tag {
        "a" => (
            quote!(StyledAnchorProps),
            quote! {
                href={props.href.clone()}
                target={props.target.clone()}
                rel={props.rel.clone()}
            },
            false,
        ),
        "button" => (
            quote!(StyledButtonProps),
            quote! {
                type={props.etype.clone()}
                disabled={props.disabled}
            },
            false,
        ),
        "form" => (
            quote!(StyledFormProps),
            quote! {
                action={props.action.clone()}
                method={props.method.clone()}
            },
            false,
        ),
        "img" => (
            quote!(StyledImgProps),
            quote! {
                src={props.src.clone()}
                alt={props.alt.clone()}
                width={props.width.clone()}
                height={props.height.clone()}
            },
            true,
        ),
        "input" => (
            quote!(StyledInputProps),
            quote! {
                type={props.etype.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                checked={props.checked}
                disabled={props.disabled}
                readonly={props.readonly}
            },
            true,
        ),
        other if is_void_tag(other) => (quote!(StyledVoidComponentProps), quote!(), true),
        _ => (quote!(StyledComponentProps), quote!(), false),
    }
}

#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentInput);
    let (class_name, css_string) = build_css(&input.css);
    let component_name = &input.name;
    let tag = &input.tag;

    let (props_ident, extra_attrs, is_void) = props_for_tag(&tag.to_string());
    codegen(
        component_name,
        tag,
        class_name,
        css_string,
        props_ident,
        extra_attrs,
        is_void,
    )
}
