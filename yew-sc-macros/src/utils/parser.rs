use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct StyleParser;

impl StyleParser {
    pub fn block_to_css(block: &syn::Block) -> String {
        block
            .stmts
            .iter()
            .map(|stmt| {
                quote::quote!(#stmt)
                    .to_string()
                    .replace(" =", ":")
                    .replace("\"", " ")
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn hash_css(css: &str) -> String {
        let mut hasher = DefaultHasher::new();
        css.hash(&mut hasher);
        format!("sc-{:x}", hasher.finish())
    }
}
