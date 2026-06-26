use proc_macro2::Span;
use syn::Result;

const KNOWN_UNITS: &[&str] = &[
    "px", "em", "rem", "vh", "vw", "vmin", "vmax", "pt", "pc", "ch", "ex", "cm", "mm", "in", "fr",
    "s", "ms", "deg", "rad", "turn",
];

pub fn validate_unit(suffix: &str, span: Span) -> Result<()> {
    if suffix.is_empty() || KNOWN_UNITS.contains(&suffix) {
        Ok(())
    } else {
        Err(syn::Error::new(
            span,
            format!("`{suffix}` is not a recognized CSS unit"),
        ))
    }
}
