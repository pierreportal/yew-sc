#[cfg(not(feature = "static-extract"))]
mod runtime {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    use web_sys::window;

    static STYLE_REGISTRY: LazyLock<Mutex<HashMap<String, String>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    fn register(class: &str, css: &str) {
        let mut styles = STYLE_REGISTRY.lock().unwrap();
        styles.entry(class.to_string()).or_insert(css.to_string());
    }

    fn inject_style() {
        let mut output = String::new();
        let styles = STYLE_REGISTRY.lock().unwrap();
        let mut entries: Vec<_> = styles.iter().collect();

        entries.sort_by_key(|(k, _)| *k); // deterministic order

        for (_class, css) in entries {
            output.push_str(css);
        }
        let document = window().unwrap().document().unwrap();

        if let Some(existing) = document.get_element_by_id("yew-styles") {
            existing.set_inner_html(&output);
        } else {
            let style_el = document.create_element("style").unwrap();
            style_el.set_id("yew-styles");
            style_el.set_inner_html(&output);
            document.head().unwrap().append_child(&style_el).unwrap();
        }
    }

    pub fn register_style(a: &str, b: &str) {
        register(a, b);
        inject_style();
    }
}

#[cfg(feature = "static-extract")]
mod runtime {
    /// No-op under `static-extract`: CSS lives in the wasm custom section
    /// instead, and is harvested by the `yew-sc-extract` xtask.
    #[inline(always)]
    pub fn register_style(_class: &str, _css: &str) {}
}

pub use runtime::register_style;

/// Macro entry point used by the proc-macro. With `static-extract` enabled,
/// the CSS argument is *not* tokenized — neither the string nor the call site
/// reaches the wasm binary. Without it, this forwards to the runtime
/// registry, which is what powers the in-browser `<style>` injection.
#[cfg(not(feature = "static-extract"))]
#[macro_export]
macro_rules! __yew_sc_register_style {
    ($class:expr, $css:expr) => {
        $crate::registry::register_style($class, $css)
    };
}

#[cfg(feature = "static-extract")]
#[macro_export]
macro_rules! __yew_sc_register_style {
    ($class:expr, $css:expr) => {
        ()
    };
}

/// Embed a CSS payload into the `yew_sc_css` wasm custom section. The
/// extractor walks this section to assemble `dist/yew-sc.css`. Without the
/// feature, the macro expands to nothing so the bytes stay out of the
/// binary.
#[cfg(feature = "static-extract")]
#[macro_export]
macro_rules! __yew_sc_embed_css {
    ($len:expr, $bytes:expr) => {
        const _: () = {
            #[used]
            #[unsafe(link_section = "yew_sc_css")]
            static _ENTRY: [u8; $len] = $bytes;
        };
    };
}

#[cfg(not(feature = "static-extract"))]
#[macro_export]
macro_rules! __yew_sc_embed_css {
    ($len:expr, $bytes:expr) => {};
}
