use proc_macro::TokenStream;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct StyledComponentInput {
    pub name: syn::Ident,
    pub tag: syn::Ident,
    pub css: syn::Block,
}

impl Parse for StyledComponentInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        let tag: syn::Ident = input.parse()?;
        let css: syn::Block = input.parse()?;
        Ok(Self { name, tag, css })
    }
}

fn block_to_css(block: &syn::Block) -> String {
    block
        .stmts
        .iter()
        .map(|stmt| {
            let s = quote::quote!(#stmt)
                .to_string()
                .replace(" =", ":")
                .replace("\"", " ");
            s
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    css.hash(&mut hasher);
    format!("sc-{:x}", hasher.finish())
}

#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyledComponentInput);
    let css_string = block_to_css(&input.css);
    let class_name = hash_css(&css_string);
    let component_name = &input.name;
    let tag = &input.tag;

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
