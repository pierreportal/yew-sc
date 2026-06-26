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
