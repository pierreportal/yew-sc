use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro2::{Span, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitFloat, LitInt, LitStr, Result, Token, braced, parenthesized};

pub struct CssBlock {
    pub items: Vec<CssItem>,
}

pub enum CssItem {
    Decl(CssDeclaration),
    Nested(NestedRule),
}

pub struct NestedRule {
    pub selector: String,
    pub block: CssBlock,
}

pub struct CssDeclaration {
    pub property: Ident,
    pub value: CssValue,
}

pub enum CssValue {
    Str(LitStr),
    Keyword(Ident),
    Int(LitInt),
    Float(LitFloat),
    Function { name: Ident, args: Vec<CssValue> },
}

const KNOWN_PROPERTIES: &[&str] = &[
    "align-items",
    "align-self",
    "background",
    "background-color",
    "background-image",
    "border",
    "border-bottom",
    "border-color",
    "border-left",
    "border-radius",
    "border-right",
    "border-style",
    "border-top",
    "border-width",
    "bottom",
    "box-shadow",
    "box-sizing",
    "color",
    "cursor",
    "display",
    "flex",
    "flex-basis",
    "flex-direction",
    "flex-grow",
    "flex-shrink",
    "flex-wrap",
    "font",
    "font-family",
    "font-size",
    "font-style",
    "font-weight",
    "gap",
    "grid",
    "grid-area",
    "grid-column",
    "grid-gap",
    "grid-row",
    "grid-template",
    "grid-template-columns",
    "grid-template-rows",
    "height",
    "justify-content",
    "left",
    "letter-spacing",
    "line-height",
    "list-style",
    "margin",
    "margin-bottom",
    "margin-left",
    "margin-right",
    "margin-top",
    "max-height",
    "max-width",
    "min-height",
    "min-width",
    "opacity",
    "outline",
    "outline-color",
    "outline-offset",
    "outline-style",
    "outline-width",
    "overflow",
    "overflow-x",
    "overflow-y",
    "padding",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "padding-top",
    "position",
    "right",
    "text-align",
    "text-decoration",
    "text-overflow",
    "text-transform",
    "top",
    "transform",
    "transition",
    "user-select",
    "vertical-align",
    "visibility",
    "white-space",
    "width",
    "word-break",
    "z-index",
];

const KNOWN_UNITS: &[&str] = &[
    "px", "em", "rem", "vh", "vw", "vmin", "vmax", "pt", "pc", "ch", "ex", "cm", "mm", "in", "fr",
    "s", "ms", "deg", "rad", "turn",
];

const KNOWN_FUNCTIONS: &[&str] = &[
    "rgb",
    "rgba",
    "hsl",
    "hsla",
    "calc",
    "var",
    "url",
    "linear-gradient",
    "radial-gradient",
    "conic-gradient",
    "translate",
    "translateX",
    "translateY",
    "translateZ",
    "translate3d",
    "rotate",
    "rotateX",
    "rotateY",
    "rotateZ",
    "scale",
    "scaleX",
    "scaleY",
    "scaleZ",
    "skew",
    "skewX",
    "skewY",
    "matrix",
    "matrix3d",
    "cubic-bezier",
    "steps",
    "min",
    "max",
    "clamp",
];

fn ident_to_css(ident: &Ident) -> String {
    ident.to_string().replace('_', "-")
}

fn validate_property(ident: &Ident) -> Result<()> {
    let name = ident_to_css(ident);
    if KNOWN_PROPERTIES.contains(&name.as_str()) {
        Ok(())
    } else {
        Err(syn::Error::new(
            ident.span(),
            format!("`{name}` is not a recognized CSS property"),
        ))
    }
}

fn validate_unit(suffix: &str, span: Span) -> Result<()> {
    if suffix.is_empty() || KNOWN_UNITS.contains(&suffix) {
        Ok(())
    } else {
        Err(syn::Error::new(
            span,
            format!("`{suffix}` is not a recognized CSS unit"),
        ))
    }
}

fn validate_function(ident: &Ident) -> Result<()> {
    let name = ident_to_css(ident);
    if KNOWN_FUNCTIONS.contains(&name.as_str()) {
        Ok(())
    } else {
        Err(syn::Error::new(
            ident.span(),
            format!("`{name}` is not a recognized CSS function"),
        ))
    }
}

fn parse_selector(input: ParseStream) -> Result<String> {
    let mut tokens: Vec<TokenTree> = Vec::new();
    while !input.is_empty() && !input.peek(syn::token::Brace) {
        let tt: TokenTree = input.parse()?;
        tokens.push(tt);
    }
    if input.is_empty() {
        return Err(input.error("expected `{` after nested selector"));
    }

    fn is_glue_punct(tt: &TokenTree) -> bool {
        // Punctuation that should be glued to adjacent tokens with no whitespace,
        // e.g. `:` in `:hover`, `.` in `.title`, `#` in `#id`, `&` itself.
        matches!(
            tt,
            TokenTree::Punct(p) if matches!(p.as_char(), ':' | '.' | '#' | '&' | '*' | '%')
        )
    }

    let mut out = String::new();
    for (i, tt) in tokens.iter().enumerate() {
        if i > 0 {
            let prev = &tokens[i - 1];
            let glue = is_glue_punct(prev) || is_glue_punct(tt);
            if !glue {
                out.push(' ');
            }
        }
        out.push_str(&tt.to_string());
    }
    Ok(out)
}

fn parse_block_items(content: ParseStream) -> Result<Vec<CssItem>> {
    let mut items = Vec::new();
    while !content.is_empty() {
        if content.peek(Token![&]) {
            let selector = parse_selector(content)?;
            let inner;
            braced!(inner in content);
            let block = CssBlock {
                items: parse_block_items(&inner)?,
            };
            items.push(CssItem::Nested(NestedRule { selector, block }));
        } else {
            items.push(CssItem::Decl(content.parse()?));
        }
    }
    Ok(items)
}

impl Parse for CssBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        Ok(CssBlock {
            items: parse_block_items(&content)?,
        })
    }
}

impl Parse for CssDeclaration {
    fn parse(input: ParseStream) -> Result<Self> {
        let property: Ident = input.parse()?;
        validate_property(&property)?;
        input.parse::<Token![=]>()?;
        let value: CssValue = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(CssDeclaration { property, value })
    }
}

impl Parse for CssValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            Ok(CssValue::Str(input.parse()?))
        } else if lookahead.peek(LitInt) {
            let lit: LitInt = input.parse()?;
            validate_unit(lit.suffix(), lit.span())?;
            Ok(CssValue::Int(lit))
        } else if lookahead.peek(LitFloat) {
            let lit: LitFloat = input.parse()?;
            validate_unit(lit.suffix(), lit.span())?;
            Ok(CssValue::Float(lit))
        } else if lookahead.peek(Ident) {
            let name: Ident = input.parse()?;
            if input.peek(syn::token::Paren) {
                validate_function(&name)?;
                let content;
                parenthesized!(content in input);
                let args: Punctuated<CssValue, Token![,]> =
                    content.parse_terminated(CssValue::parse, Token![,])?;
                Ok(CssValue::Function {
                    name,
                    args: args.into_iter().collect(),
                })
            } else {
                Ok(CssValue::Keyword(name))
            }
        } else {
            Err(lookahead.error())
        }
    }
}

impl CssValue {
    pub fn to_css(&self) -> String {
        match self {
            CssValue::Str(s) => s.value(),
            CssValue::Keyword(id) => ident_to_css(id),
            CssValue::Int(i) => i.token().to_string(),
            CssValue::Float(f) => f.token().to_string(),
            CssValue::Function { name, args } => {
                let args_str = args
                    .iter()
                    .map(CssValue::to_css)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", ident_to_css(name), args_str)
            }
        }
    }
}

impl CssBlock {
    pub fn to_rules(&self, parent: &str) -> Vec<(String, String)> {
        let mut own = Vec::new();
        let mut children = Vec::new();
        for item in &self.items {
            match item {
                CssItem::Decl(d) => {
                    own.push(format!(
                        "{}: {};",
                        ident_to_css(&d.property),
                        d.value.to_css()
                    ));
                }
                CssItem::Nested(rule) => {
                    let resolved = rule.selector.replace('&', parent);
                    children.extend(rule.block.to_rules(&resolved));
                }
            }
        }
        let mut rules = Vec::new();
        if !own.is_empty() {
            rules.push((parent.to_string(), own.join(" ")));
        }
        rules.extend(children);
        rules
    }
}

pub fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    css.hash(&mut hasher);
    format!("sc-{:x}", hasher.finish())
}
