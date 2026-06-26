use syn::{Ident, Result};

use super::to_css::ToCss;

const KNOWN_PROPERTIES: &[&str] = &[
    "align-items",
    "align-self",
    "animation",
    "animation-delay",
    "animation-direction",
    "animation-duration",
    "animation-fill-mode",
    "animation-iteration-count",
    "animation-name",
    "animation-play-state",
    "animation-timing-function",
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
    "text-shadow",
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

pub fn validate_property(ident: &Ident) -> Result<()> {
    let name = ident.to_css();
    if KNOWN_PROPERTIES.contains(&name.as_str()) {
        Ok(())
    } else {
        Err(syn::Error::new(
            ident.span(),
            format!("`{name}` is not a recognized CSS property"),
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
    fn known_property_accepted() {
        assert!(validate_property(&ident("color")).is_ok());
        assert!(validate_property(&ident("background")).is_ok());
    }

    #[test]
    fn underscores_are_translated_to_dashes() {
        // `border_radius` is `border-radius` after to_css()
        assert!(validate_property(&ident("border_radius")).is_ok());
        assert!(validate_property(&ident("font_size")).is_ok());
    }

    #[test]
    fn unknown_property_is_rejected() {
        let err = validate_property(&ident("colur")).unwrap_err();
        assert!(err.to_string().contains("colur"));
        assert!(err.to_string().contains("not a recognized CSS property"));
    }
}
