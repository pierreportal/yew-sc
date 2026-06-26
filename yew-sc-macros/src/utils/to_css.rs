use syn::{Ident, LitFloat, LitInt, LitStr};

pub trait ToCss {
    fn to_css(&self) -> String;
}

impl ToCss for Ident {
    fn to_css(&self) -> String {
        self.to_string().replace('_', "-")
    }
}

impl ToCss for LitStr {
    fn to_css(&self) -> String {
        self.value()
    }
}

impl ToCss for LitInt {
    fn to_css(&self) -> String {
        self.token().to_string()
    }
}

impl ToCss for LitFloat {
    fn to_css(&self) -> String {
        self.token().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::parse_str;

    #[test]
    fn ident_underscores_become_dashes() {
        let id = Ident::new("border_top_left_radius", Span::call_site());
        assert_eq!(id.to_css(), "border-top-left-radius");
    }

    #[test]
    fn ident_without_underscores_unchanged() {
        let id = Ident::new("color", Span::call_site());
        assert_eq!(id.to_css(), "color");
    }

    #[test]
    fn lit_str_yields_unescaped_value() {
        let s: LitStr = parse_str(r#""hello world""#).unwrap();
        assert_eq!(s.to_css(), "hello world");
    }

    #[test]
    fn lit_int_preserves_suffix() {
        let i: LitInt = parse_str("12px").unwrap();
        assert_eq!(i.to_css(), "12px");
    }

    #[test]
    fn lit_float_preserves_suffix() {
        let f: LitFloat = parse_str("1.5rem").unwrap();
        assert_eq!(f.to_css(), "1.5rem");
    }
}
