use syn::{Ident, Result};

use super::to_css::ToCss;

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

pub fn validate_function(ident: &Ident) -> Result<()> {
    let name = ident.to_css();
    if KNOWN_FUNCTIONS.contains(&name.as_str()) {
        Ok(())
    } else {
        Err(syn::Error::new(
            ident.span(),
            format!("`{name}` is not a recognized CSS function"),
        ))
    }
}
