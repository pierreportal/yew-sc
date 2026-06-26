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

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[test]
    fn empty_suffix_is_unitless_ok() {
        assert!(validate_unit("", Span::call_site()).is_ok());
    }

    #[test]
    fn every_known_unit_is_accepted() {
        for u in KNOWN_UNITS {
            assert!(
                validate_unit(u, Span::call_site()).is_ok(),
                "expected `{u}` to be accepted"
            );
        }
    }

    #[test]
    fn unknown_unit_is_rejected_with_message() {
        let err = validate_unit("foo", Span::call_site()).unwrap_err();
        assert!(err.to_string().contains("foo"));
        assert!(err.to_string().contains("not a recognized CSS unit"));
    }

    #[test]
    fn unit_match_is_case_sensitive() {
        assert!(validate_unit("PX", Span::call_site()).is_err());
    }
}
