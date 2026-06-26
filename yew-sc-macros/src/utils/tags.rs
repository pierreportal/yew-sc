use syn::{Ident, Result};

const KNOWN_TAGS: &[&str] = &[
    "a",
    "abbr",
    "address",
    "article",
    "aside",
    "audio",
    "b",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "button",
    "canvas",
    "caption",
    "cite",
    "code",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "div",
    "dl",
    "dt",
    "em",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "html",
    "i",
    "iframe",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "main",
    "map",
    "mark",
    "menu",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "slot",
    "small",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "u",
    "ul",
    "var",
    "video",
    // void elements below
    "area",
    "base",
    "br",
    "col",
    "embed",
    "hr",
    "img",
    "input",
    "link",
    "meta",
    "source",
    "track",
    "wbr",
];

const VOID_TAGS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr",
];

pub fn validate_tag(ident: &Ident) -> Result<()> {
    let name = ident.to_string();
    if KNOWN_TAGS.contains(&name.as_str()) {
        Ok(())
    } else {
        Err(syn::Error::new(
            ident.span(),
            format!("`{name}` is not a recognized HTML tag"),
        ))
    }
}

pub fn is_void_tag(name: &str) -> bool {
    VOID_TAGS.contains(&name)
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
    fn known_tag_is_accepted() {
        assert!(validate_tag(&ident("div")).is_ok());
        assert!(validate_tag(&ident("h1")).is_ok());
        assert!(validate_tag(&ident("img")).is_ok());
    }

    #[test]
    fn unknown_tag_is_rejected_with_message() {
        let err = validate_tag(&ident("zonk")).unwrap_err();
        assert!(err.to_string().contains("zonk"));
        assert!(err.to_string().contains("not a recognized HTML tag"));
    }

    #[test]
    fn void_tags_are_void() {
        for t in VOID_TAGS {
            assert!(is_void_tag(t), "expected `{t}` to be void");
        }
    }

    #[test]
    fn non_void_tags_are_not_void() {
        for t in ["div", "span", "button", "p", "h1", "a"] {
            assert!(!is_void_tag(t), "expected `{t}` to not be void");
        }
    }
}
