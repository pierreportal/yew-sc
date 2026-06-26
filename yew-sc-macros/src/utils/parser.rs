use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, LitFloat, LitInt, LitStr, Result, Token, braced, parenthesized};

use super::functions::validate_function;
use super::properties::validate_property;
use super::to_css::ToCss;
use super::units::validate_unit;

pub struct Keyframes {
    pub name: Ident,
    pub stops: Vec<KeyframeStop>,
}

pub struct KeyframeStop {
    pub selector: String,
    pub decls: Vec<CssDeclaration>,
}

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
    Var(Ident),
    Expr(syn::Expr),
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

fn starts_with_ampersand(input: ParseStream) -> bool {
    input
        .cursor()
        .punct()
        .map(|(p, _)| p.as_char() == '&')
        .unwrap_or(false)
}

fn parse_block_items(content: ParseStream) -> Result<Vec<CssItem>> {
    let mut items = Vec::new();
    while !content.is_empty() {
        if starts_with_ampersand(content) {
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

fn parse_stop_selector(input: ParseStream) -> Result<String> {
    let mut tokens: Vec<TokenTree> = Vec::new();
    while !input.is_empty() && !input.peek(syn::token::Brace) {
        let tt: TokenTree = input.parse()?;
        tokens.push(tt);
    }
    if tokens.is_empty() {
        return Err(input.error("expected keyframe stop (e.g. `from`, `to`, or `50%`)"));
    }
    let mut out = String::new();
    for (i, tt) in tokens.iter().enumerate() {
        if i > 0 {
            let glue =
                matches!(tt, TokenTree::Punct(p) if p.as_char() == '%' || p.as_char() == ',');
            if !glue {
                out.push(' ');
            }
        }
        out.push_str(&tt.to_string());
    }
    Ok(out)
}

impl Parse for Keyframes {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let body;
        braced!(body in input);
        let mut stops = Vec::new();
        while !body.is_empty() {
            let selector = parse_stop_selector(&body)?;
            let inner;
            braced!(inner in body);
            let mut decls = Vec::new();
            while !inner.is_empty() {
                decls.push(inner.parse::<CssDeclaration>()?);
            }
            stops.push(KeyframeStop { selector, decls });
        }
        Ok(Keyframes { name, stops })
    }
}

impl Keyframes {
    pub fn hashed_name(&self) -> String {
        let body = self.body_css();
        let mut hasher = DefaultHasher::new();
        body.hash(&mut hasher);
        format!("{}-{:x}", self.name, hasher.finish())
    }

    fn body_css(&self) -> String {
        let mut out = String::new();
        for stop in &self.stops {
            out.push_str(&stop.selector);
            out.push_str(" { ");
            for d in &stop.decls {
                out.push_str(&format!("{}: {}; ", d.property.to_css(), d.value.to_css()));
            }
            out.push('}');
            out.push(' ');
        }
        out.trim_end().to_string()
    }

    pub fn rendered_css(&self, hashed: &str) -> String {
        format!("@keyframes {} {{ {} }}", hashed, self.body_css())
    }
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

fn peek_dollar(input: ParseStream) -> bool {
    input
        .cursor()
        .punct()
        .map(|(p, _)| p.as_char() == '$')
        .unwrap_or(false)
}

impl Parse for CssValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if peek_dollar(input) {
            input.parse::<Token![$]>()?;
            if input.peek(syn::token::Brace) {
                let inner;
                braced!(inner in input);
                let expr: syn::Expr = inner.parse()?;
                return Ok(CssValue::Expr(expr));
            }
            let name: Ident = input.parse()?;
            return Ok(CssValue::Var(name));
        }
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

impl ToCss for CssValue {
    fn to_css(&self) -> String {
        match self {
            CssValue::Str(s) => s.to_css(),
            CssValue::Keyword(id) => id.to_css(),
            CssValue::Int(i) => i.to_css(),
            CssValue::Float(f) => f.to_css(),
            CssValue::Function { name, args } => {
                let args_str = args
                    .iter()
                    .map(CssValue::to_css)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name.to_css(), args_str)
            }
            CssValue::Var(id) => format!("var(--{})", id.to_css()),
            CssValue::Expr(_) => {
                panic!("CssValue::Expr should be lowered to CssValue::Var before to_css()")
            }
        }
    }
}

impl CssValue {
    fn collect_vars(&self, out: &mut Vec<Ident>) {
        match self {
            CssValue::Var(id) => out.push(id.clone()),
            CssValue::Function { args, .. } => {
                for arg in args {
                    arg.collect_vars(out);
                }
            }
            _ => {}
        }
    }

    fn lower_exprs(&mut self, exprs: &mut Vec<(Ident, syn::Expr)>) {
        match self {
            CssValue::Expr(_) => {
                let idx = exprs.len();
                let ident = Ident::new(
                    &format!("__yew_sc_expr_{}", idx),
                    proc_macro2::Span::call_site(),
                );
                let placeholder = CssValue::Var(ident.clone());
                let old = std::mem::replace(self, placeholder);
                if let CssValue::Expr(expr) = old {
                    exprs.push((ident, expr));
                }
            }
            CssValue::Function { args, .. } => {
                for arg in args {
                    arg.lower_exprs(exprs);
                }
            }
            _ => {}
        }
    }
}

impl CssBlock {
    pub fn collect_vars(&self) -> Vec<Ident> {
        let mut out = Vec::new();
        self.collect_vars_into(&mut out);
        let mut seen = std::collections::BTreeSet::new();
        out.retain(|id| seen.insert(id.to_string()));
        out
    }

    fn collect_vars_into(&self, out: &mut Vec<Ident>) {
        for item in &self.items {
            match item {
                CssItem::Decl(d) => d.value.collect_vars(out),
                CssItem::Nested(rule) => rule.block.collect_vars_into(out),
            }
        }
    }

    pub fn lower_exprs(&mut self) -> Vec<(Ident, syn::Expr)> {
        let mut out = Vec::new();
        self.lower_exprs_into(&mut out);
        out
    }

    fn lower_exprs_into(&mut self, out: &mut Vec<(Ident, syn::Expr)>) {
        for item in &mut self.items {
            match item {
                CssItem::Decl(d) => d.value.lower_exprs(out),
                CssItem::Nested(rule) => rule.block.lower_exprs_into(out),
            }
        }
    }

    pub fn rewrite_keyframe_refs(&mut self, names: &BTreeMap<String, String>) {
        if names.is_empty() {
            return;
        }
        for item in &mut self.items {
            match item {
                CssItem::Decl(d) => {
                    let prop = d.property.to_css();
                    if prop == "animation" || prop == "animation-name" {
                        rewrite_value(&mut d.value, names);
                    }
                }
                CssItem::Nested(rule) => rule.block.rewrite_keyframe_refs(names),
            }
        }
    }

    pub fn to_rules(&self, parent: &str) -> Vec<(String, String)> {
        let mut own = Vec::new();
        let mut children = Vec::new();
        for item in &self.items {
            match item {
                CssItem::Decl(d) => {
                    own.push(format!("{}: {};", d.property.to_css(), d.value.to_css()));
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

fn rewrite_value(value: &mut CssValue, names: &BTreeMap<String, String>) {
    match value {
        CssValue::Keyword(id) => {
            let key = id.to_string();
            if let Some(hashed) = names.get(&key) {
                *value = CssValue::Str(LitStr::new(hashed, id.span()));
            }
        }
        CssValue::Str(s) => {
            let raw = s.value();
            let replaced = replace_idents(&raw, names);
            if replaced != raw {
                *value = CssValue::Str(LitStr::new(&replaced, s.span()));
            }
        }
        CssValue::Function { args, .. } => {
            for arg in args {
                rewrite_value(arg, names);
            }
        }
        _ => {}
    }
}

fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

fn replace_idents(input: &str, names: &BTreeMap<String, String>) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < bytes.len() && is_ident_char(bytes[i] as char) {
                i += 1;
            }
            let word = &input[start..i];
            let prev_ok = start == 0 || !is_ident_char(bytes[start - 1] as char);
            if prev_ok && let Some(hashed) = names.get(word) {
                out.push_str(hashed);
                continue;
            }
            out.push_str(word);
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

pub fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    css.hash(&mut hasher);
    format!("ysc-{:x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;

    #[test]
    fn hash_is_deterministic_and_prefixed() {
        let a = hash_css("color: red;");
        let b = hash_css("color: red;");
        assert_eq!(a, b);
        assert!(a.starts_with("ysc-"));
    }

    #[test]
    fn hash_differs_for_different_input() {
        assert_ne!(hash_css("color: red;"), hash_css("color: blue;"));
    }

    #[test]
    fn css_block_parses_simple_decls() {
        let block: CssBlock = parse_str("{ color = red; padding = 10px; }").unwrap();
        let rules = block.to_rules(".self");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].0, ".self");
        assert_eq!(rules[0].1, "color: red; padding: 10px;");
    }

    #[test]
    fn css_block_nests_via_ampersand() {
        let block: CssBlock = parse_str("{ color = blue; &:hover { color = red; } }").unwrap();
        let rules = block.to_rules(".self");
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].0, ".self");
        assert_eq!(rules[0].1, "color: blue;");
        // selector preserves `&` glued to `:hover` and replaces `&` with parent.
        assert_eq!(rules[1].0, ".self:hover");
        assert_eq!(rules[1].1, "color: red;");
    }

    #[test]
    fn vars_become_css_var_calls() {
        let block: CssBlock = parse_str("{ background = $bg; }").unwrap();
        let rules = block.to_rules(".x");
        assert_eq!(rules[0].1, "background: var(--bg);");
    }

    #[test]
    fn vars_in_underscored_idents_dash_in_var_name() {
        let block: CssBlock = parse_str("{ background = $bg_hover; }").unwrap();
        let rules = block.to_rules(".x");
        // Var idents go through to_css → underscores become dashes.
        assert!(rules[0].1.contains("var(--bg-hover)"));
    }

    #[test]
    fn collect_vars_dedupes_and_orders() {
        let block: CssBlock =
            parse_str("{ color = $a; background = $b; border_color = $a; }").unwrap();
        let vars: Vec<String> = block
            .collect_vars()
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        assert_eq!(vars, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn collect_vars_descends_into_nested() {
        let block: CssBlock = parse_str("{ color = $a; &:hover { background = $b; } }").unwrap();
        let names: Vec<String> = block
            .collect_vars()
            .into_iter()
            .map(|i| i.to_string())
            .collect();
        assert!(names.contains(&"a".to_string()));
        assert!(names.contains(&"b".to_string()));
    }

    #[test]
    fn function_call_renders_args() {
        let block: CssBlock = parse_str("{ background = rgb(10, 20, 30); }").unwrap();
        let rules = block.to_rules(".x");
        assert_eq!(rules[0].1, "background: rgb(10, 20, 30);");
    }

    #[test]
    fn keyframes_parses_and_renders_hashed_name() {
        let kf: Keyframes = parse_str(
            "spin { from { transform = rotate(0deg); } to { transform = rotate(360deg); } }",
        )
        .unwrap();
        let hashed = kf.hashed_name();
        assert!(hashed.starts_with("spin-"));
        let rendered = kf.rendered_css(&hashed);
        assert!(rendered.starts_with("@keyframes spin-"));
        assert!(rendered.contains("from {"));
        assert!(rendered.contains("transform: rotate(0deg);"));
        assert!(rendered.contains("to {"));
    }

    #[test]
    fn keyframes_hashed_name_is_deterministic() {
        let a: Keyframes =
            parse_str("blink { from { opacity = 0; } to { opacity = 1; } }").unwrap();
        let b: Keyframes =
            parse_str("blink { from { opacity = 0; } to { opacity = 1; } }").unwrap();
        assert_eq!(a.hashed_name(), b.hashed_name());
    }

    #[test]
    fn rewrite_keyframe_refs_replaces_animation_name_keyword() {
        let mut block: CssBlock = parse_str("{ animation_name = spin; color = red; }").unwrap();
        let mut names: BTreeMap<String, String> = BTreeMap::new();
        names.insert("spin".to_string(), "spin-abc123".to_string());
        block.rewrite_keyframe_refs(&names);
        let rules = block.to_rules(".x");
        assert!(rules[0].1.contains("animation-name: spin-abc123"));
        assert!(rules[0].1.contains("color: red"));
    }

    #[test]
    fn rewrite_keyframe_refs_inside_animation_shorthand_string() {
        let mut block: CssBlock =
            parse_str("{ animation = \"pulse 1.6s ease-in-out infinite\"; }").unwrap();
        let mut names: BTreeMap<String, String> = BTreeMap::new();
        names.insert("pulse".to_string(), "pulse-deadbeef".to_string());
        block.rewrite_keyframe_refs(&names);
        let rules = block.to_rules(".x");
        assert!(
            rules[0]
                .1
                .contains("pulse-deadbeef 1.6s ease-in-out infinite")
        );
    }

    #[test]
    fn rewrite_keyframe_refs_only_touches_animation_properties() {
        let mut block: CssBlock = parse_str("{ color = spin; }").unwrap();
        let mut names: BTreeMap<String, String> = BTreeMap::new();
        names.insert("spin".to_string(), "spin-xxx".to_string());
        block.rewrite_keyframe_refs(&names);
        let rules = block.to_rules(".x");
        // `color = spin` is left untouched because the property isn't `animation*`.
        assert_eq!(rules[0].1, "color: spin;");
    }

    #[test]
    fn lower_exprs_replaces_inline_exprs_with_synthetic_vars() {
        let mut block: CssBlock =
            parse_str("{ background = ${ if true { \"red\" } else { \"blue\" } }; }").unwrap();
        let exprs = block.lower_exprs();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0].0.to_string(), "__yew_sc_expr_0");
        // After lowering, the rule references the synthetic var. Underscores
        // in the ident are translated to dashes by ToCss, so the leading `__`
        // becomes `--`, yielding `----yew-sc-expr-0`. The codegen side mirrors
        // this when setting the inline custom property, so the two ends agree.
        let rules = block.to_rules(".x");
        assert!(
            rules[0].1.contains("var(----yew-sc-expr-0)"),
            "got: {}",
            rules[0].1
        );
    }

    #[test]
    fn invalid_property_fails_to_parse() {
        let err = match syn::parse_str::<CssBlock>("{ colur = red; }") {
            Ok(_) => panic!("expected parse error"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("not a recognized CSS property"));
    }

    #[test]
    fn invalid_unit_fails_to_parse() {
        let err = match syn::parse_str::<CssBlock>("{ padding = 10ft; }") {
            Ok(_) => panic!("expected parse error"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("not a recognized CSS unit"));
    }

    #[test]
    fn invalid_function_fails_to_parse() {
        let err = match syn::parse_str::<CssBlock>("{ color = bogus(1, 2); }") {
            Ok(_) => panic!("expected parse error"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("not a recognized CSS function"));
    }
}
