mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Fields, ItemStruct, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
};
use utils::parser::{CssBlock, hash_css};
use utils::tags::{is_void_tag, validate_tag};

const BASE_FIELD_NAMES: &[&str] = &[
    "children",
    "class",
    "onclick",
    "id",
    "title",
    "hidden",
    "tabindex",
    "role",
];

fn base_field(name: &str) -> syn::Field {
    match name {
        "children" => parse_quote! { #[prop_or_default] pub children: ::yew::Children },
        "class" => parse_quote! { #[prop_or_default] pub class: ::yew::Classes },
        "onclick" => {
            parse_quote! { #[prop_or_default] pub onclick: ::yew::Callback<::yew::MouseEvent> }
        }
        "id" => parse_quote! { #[prop_or_default] pub id: Option<::yew::AttrValue> },
        "title" => parse_quote! { #[prop_or_default] pub title: Option<::yew::AttrValue> },
        "hidden" => parse_quote! { #[prop_or_default] pub hidden: bool },
        "tabindex" => parse_quote! { #[prop_or_default] pub tabindex: Option<::yew::AttrValue> },
        "role" => parse_quote! { #[prop_or_default] pub role: Option<::yew::AttrValue> },
        _ => unreachable!(),
    }
}

#[proc_macro_attribute]
pub fn styled_props(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemStruct);

    let Fields::Named(fields) = &mut item.fields else {
        return syn::Error::new_spanned(
            &item,
            "`#[styled_props]` requires a struct with named fields",
        )
        .to_compile_error()
        .into();
    };

    let existing: std::collections::HashSet<String> = fields
        .named
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
        .collect();

    let mut prepended: Vec<syn::Field> = BASE_FIELD_NAMES
        .iter()
        .filter(|n| !existing.contains(**n))
        .map(|n| base_field(n))
        .collect();
    prepended.extend(fields.named.iter().cloned());
    fields.named = prepended.into_iter().collect();

    let has_props_derive = item.attrs.iter().any(|a| {
        a.path()
            .segments
            .last()
            .map(|s| s.ident == "Properties")
            .unwrap_or(false)
    });
    if !has_props_derive {
        item.attrs
            .push(parse_quote!(#[derive(::yew::Properties, ::std::cmp::PartialEq)]));
    }

    quote!(#item).into()
}

struct StyledComponentInput {
    pub name: syn::Ident,
    pub user_props: Option<syn::Ident>,
    pub tag: syn::Ident,
    pub css: CssBlock,
}

impl Parse for StyledComponentInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let user_props = if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let ident: syn::Ident = input.parse()?;
            input.parse::<Token![>]>()?;
            Some(ident)
        } else {
            None
        };
        input.parse::<Token![=>]>()?;
        let tag: syn::Ident = input.parse()?;
        validate_tag(&tag)?;
        let css: CssBlock = input.parse()?;
        Ok(Self {
            name,
            user_props,
            tag,
            css,
        })
    }
}

struct StyledComponents(Vec<StyledComponentInput>);

impl Parse for StyledComponents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse::<StyledComponentInput>()?);
        }
        if items.is_empty() {
            return Err(input.error("expected at least one component declaration"));
        }
        Ok(StyledComponents(items))
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

struct TagShape {
    is_void: bool,
    extra_fields: proc_macro2::TokenStream,
    extra_attrs: proc_macro2::TokenStream,
}

fn tag_shape(tag: &str) -> TagShape {
    let is_void = is_void_tag(tag);
    let (extra_fields, extra_attrs) = match tag {
        "a" => (
            quote! {
                #[prop_or_default]
                pub href: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub target: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub rel: Option<::yew::AttrValue>,
            },
            quote! {
                href={props.href.clone()}
                target={props.target.clone()}
                rel={props.rel.clone()}
            },
        ),
        "button" => (
            quote! {
                #[prop_or_default]
                pub etype: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub disabled: bool,
            },
            quote! {
                type={props.etype.clone()}
                disabled={props.disabled}
            },
        ),
        "form" => (
            quote! {
                #[prop_or_default]
                pub action: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub method: Option<::yew::AttrValue>,
            },
            quote! {
                action={props.action.clone()}
                method={props.method.clone()}
            },
        ),
        "img" => (
            quote! {
                #[prop_or_default]
                pub src: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub alt: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub width: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub height: Option<::yew::AttrValue>,
            },
            quote! {
                src={props.src.clone()}
                alt={props.alt.clone()}
                width={props.width.clone()}
                height={props.height.clone()}
            },
        ),
        "input" => (
            quote! {
                #[prop_or_default]
                pub etype: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub value: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub placeholder: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub checked: bool,
                #[prop_or_default]
                pub disabled: bool,
                #[prop_or_default]
                pub readonly: bool,
            },
            quote! {
                type={props.etype.clone()}
                value={props.value.clone()}
                placeholder={props.placeholder.clone()}
                checked={props.checked}
                disabled={props.disabled}
                readonly={props.readonly}
            },
        ),
        _ => (quote!(), quote!()),
    };

    TagShape {
        is_void,
        extra_fields,
        extra_attrs,
    }
}

fn expand_component(mut input: StyledComponentInput) -> proc_macro2::TokenStream {
    let inline_exprs = input.css.lower_exprs();
    let (class_name, css_string) = build_css(&input.css);
    let component_name = &input.name;
    let tag = &input.tag;

    let shape = tag_shape(&tag.to_string());
    let TagShape {
        is_void,
        extra_fields,
        extra_attrs,
    } = shape;
    let extra_attrs = if input.user_props.is_some() {
        quote!()
    } else {
        extra_attrs
    };

    let prop_vars = input.css.collect_vars();
    let prop_vars: Vec<_> = prop_vars
        .into_iter()
        .filter(|id| !id.to_string().starts_with("__yew_sc_expr_"))
        .collect();

    let has_any_vars = !prop_vars.is_empty() || !inline_exprs.is_empty();

    let style_attr = if !has_any_vars {
        quote!()
    } else {
        let prop_parts = prop_vars.iter().map(|v| {
            let key = format!("--{}", v.to_string().replace('_', "-"));
            quote! {
                if let Some(val) = ::yew_sc::ToCssVar::css_var_value(&props.#v) {
                    style.push_str(#key);
                    style.push_str(": ");
                    style.push_str(&val);
                    style.push_str(";");
                }
            }
        });
        let expr_parts = inline_exprs.iter().map(|(ident, expr)| {
            let key = format!("--{}", ident.to_string().replace('_', "-"));
            quote! {
                if let Some(val) = ::yew_sc::ToCssVar::css_var_value(&{ #expr }) {
                    style.push_str(#key);
                    style.push_str(": ");
                    style.push_str(&val);
                    style.push_str(";");
                }
            }
        });
        quote! {
            let style: ::yew::AttrValue = {
                let mut style = String::new();
                #(#prop_parts)*
                #(#expr_parts)*
                style.into()
            };
        }
    };

    let style_forward = if !has_any_vars {
        quote!()
    } else {
        quote!(style={style.clone()})
    };

    let children_render = if is_void {
        quote!()
    } else {
        quote! { { for props.children.iter() } }
    };

    let element = if is_void {
        quote! {
            <#tag
                class={::yew::classes!(#class_name, props.class.clone())}
                onclick={props.onclick.clone()}
                id={props.id.clone()}
                title={props.title.clone()}
                hidden={props.hidden}
                tabindex={props.tabindex.clone()}
                role={props.role.clone()}
                #style_forward
                #extra_attrs
            />
        }
    } else {
        quote! {
            <#tag
                class={::yew::classes!(#class_name, props.class.clone())}
                onclick={props.onclick.clone()}
                id={props.id.clone()}
                title={props.title.clone()}
                hidden={props.hidden}
                tabindex={props.tabindex.clone()}
                role={props.role.clone()}
                #style_forward
                #extra_attrs
            >
                #children_render
            </#tag>
        }
    };

    let component_fn = |props_ty: &proc_macro2::TokenStream| {
        quote! {
            #[::yew::component]
            pub fn #component_name(props: &#props_ty) -> ::yew::Html {
                ::yew::use_effect(|| {
                    ::yew_sc::register_style(#class_name, #css_string)
                });
                #style_attr
                ::yew::html! { #element }
            }
        }
    };

    let expanded = if let Some(user_props) = &input.user_props {
        let props_ty = quote!(#user_props);
        let fn_tokens = component_fn(&props_ty);
        quote! { #fn_tokens }
    } else {
        let props_name = format_ident!("{}Props", component_name);
        let props_ty = quote!(#props_name);
        let fn_tokens = component_fn(&props_ty);

        let children_field = if is_void {
            quote!()
        } else {
            quote! {
                #[prop_or_default]
                pub children: ::yew::Children,
            }
        };

        let var_fields = prop_vars.iter().map(|v| {
            quote! {
                #[prop_or_default]
                pub #v: Option<::yew::AttrValue>,
            }
        });

        quote! {
            #[derive(::yew::Properties, ::std::cmp::PartialEq)]
            pub struct #props_name {
                #children_field
                #[prop_or_default]
                pub class: ::yew::Classes,
                #[prop_or_default]
                pub onclick: ::yew::Callback<::yew::MouseEvent>,
                #[prop_or_default]
                pub id: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub title: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub hidden: bool,
                #[prop_or_default]
                pub tabindex: Option<::yew::AttrValue>,
                #[prop_or_default]
                pub role: Option<::yew::AttrValue>,
                #extra_fields
                #(#var_fields)*
            }

            #fn_tokens
        }
    };

    expanded
}

#[proc_macro]
pub fn styled_component(input: TokenStream) -> TokenStream {
    let components = parse_macro_input!(input as StyledComponents);
    let expansions = components.0.into_iter().map(expand_component);
    quote!(#(#expansions)*).into()
}
