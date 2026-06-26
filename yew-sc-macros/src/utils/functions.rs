use syn::{Ident, Result};

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

pub fn validate_function(ident: &Ident) -> Result<()> {
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
