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

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;
    use syn::Ident;

    fn ident(name: &str) -> Ident {
        Ident::new(name, Span::call_site())
    }

    #[test]
    fn known_functions_accepted() {
        for n in ["rgb", "rgba", "hsl", "calc", "var", "scale", "translate"] {
            assert!(
                validate_function(&ident(n)).is_ok(),
                "expected `{n}` accepted"
            );
        }
    }

    #[test]
    fn dashed_functions_use_underscores_in_rust() {
        // `linear_gradient` in rust → `linear-gradient` after to_css().
        assert!(validate_function(&ident("linear_gradient")).is_ok());
        assert!(validate_function(&ident("cubic_bezier")).is_ok());
    }

    #[test]
    fn unknown_function_rejected() {
        let err = validate_function(&ident("rgbX")).unwrap_err();
        assert!(err.to_string().contains("rgbX"));
        assert!(err.to_string().contains("not a recognized CSS function"));
    }
}
