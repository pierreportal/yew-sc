#[cfg(not(feature = "static-extract"))]
mod runtime {
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};

    static STYLE_REGISTRY: LazyLock<Mutex<HashMap<String, String>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    /// Record a class's CSS in the process-wide registry. Pure bookkeeping —
    /// no DOM access — so this is safe to call from every render, including
    /// server-side rendering where there is no `window`/`document`.
    pub fn register_style(class: &str, css: &str) {
        let mut styles = STYLE_REGISTRY.lock().unwrap();
        styles.entry(class.to_string()).or_insert(css.to_string());
    }

    /// Concatenate every style registered so far into one deterministically
    /// ordered stylesheet. Used both for the in-browser `<style>` tag and for
    /// server-side rendering, where the caller embeds this string directly in
    /// the response instead of relying on client-side DOM injection.
    ///
    /// Note: the registry is process-wide and never cleared, so under SSR
    /// this returns every style registered by any request handled by this
    /// process so far, not just the ones on the current page. Class names
    /// are content-hashed, so this is safe (a class always maps to the same
    /// CSS) at the cost of some unused rules in the response.
    pub fn stylesheet() -> String {
        let styles = STYLE_REGISTRY.lock().unwrap();
        let mut entries: Vec<_> = styles.iter().collect();
        entries.sort_by_key(|(k, _)| *k); // deterministic order

        let mut output = String::new();
        for (_class, css) in entries {
            output.push_str(css);
        }
        output
    }

    /// Write the current stylesheet into a `<style id="yew-styles">` tag in
    /// the document. DOM-only — a no-op off `wasm32`, since `web_sys`
    /// bindings panic unconditionally on other targets rather than
    /// returning `None`. Yew never runs effects during server-side
    /// rendering, so callers that only invoke this from a `use_effect` are
    /// safe by construction; the target gate covers native builds that link
    /// this crate directly (e.g. `cargo test`, an SSR binary).
    #[cfg(target_arch = "wasm32")]
    pub fn sync_dom() {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            return;
        };
        let output = stylesheet();

        if let Some(existing) = document.get_element_by_id("yew-styles") {
            existing.set_inner_html(&output);
        } else if let Ok(style_el) = document.create_element("style") {
            style_el.set_id("yew-styles");
            style_el.set_inner_html(&output);
            if let Some(head) = document.head() {
                let _ = head.append_child(&style_el);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn sync_dom() {}

    #[cfg(test)]
    mod tests {
        use super::*;

        // The registry is a process-wide static, so tests use classes
        // unique to each test to avoid cross-test interference.

        #[test]
        fn register_and_read_back() {
            register_style("ysc-test-a", ".ysc-test-a { color: red; }");
            let sheet = stylesheet();
            assert!(sheet.contains(".ysc-test-a { color: red; }"));
        }

        #[test]
        fn registering_same_class_twice_keeps_first_value() {
            register_style("ysc-test-b", ".ysc-test-b { color: red; }");
            register_style("ysc-test-b", ".ysc-test-b { color: blue; }");
            let sheet = stylesheet();
            assert!(sheet.contains(".ysc-test-b { color: red; }"));
            assert!(!sheet.contains("blue"));
        }

        #[test]
        fn stylesheet_orders_classes_deterministically() {
            register_style("ysc-test-z", ".ysc-test-z {}");
            register_style("ysc-test-y", ".ysc-test-y {}");
            let sheet = stylesheet();
            let y_pos = sheet.find(".ysc-test-y").unwrap();
            let z_pos = sheet.find(".ysc-test-z").unwrap();
            assert!(y_pos < z_pos);
        }

        #[test]
        fn sync_dom_is_a_noop_off_wasm32() {
            // `web_sys` bindings panic unconditionally off `wasm32` (there is
            // no graceful `None`), so `sync_dom` must be gated by target
            // rather than by a runtime `window()` check. This is what makes
            // linking this crate into a native SSR binary safe.
            sync_dom();
        }
    }
}

#[cfg(feature = "static-extract")]
mod runtime {
    /// No-op under `static-extract`: CSS lives in the wasm custom section
    /// instead, and is harvested by the `yew-sc-extract` xtask.
    #[inline(always)]
    pub fn register_style(_class: &str, _css: &str) {}

    /// No-op under `static-extract`: there is nothing to embed at runtime.
    #[inline(always)]
    pub fn stylesheet() -> String {
        String::new()
    }

    /// No-op under `static-extract`: nothing was registered, so there is
    /// nothing to write to the DOM.
    #[inline(always)]
    pub fn sync_dom() {}
}

pub use runtime::{register_style, stylesheet, sync_dom};

/// Macro entry point used by the proc-macro. Called directly in every
/// component's render body (not just from an effect) so the registry is
/// populated during server-side rendering too, where Yew never runs effects.
/// With `static-extract` enabled, the CSS argument is *not* tokenized —
/// neither the string nor the call site reaches the wasm binary.
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

/// Macro entry point used by the proc-macro from inside `use_effect`: syncs
/// the accumulated registry into the DOM. Effects only run client-side (Yew
/// never runs them during SSR), so this is where the `<style>` tag write
/// belongs, separate from `__yew_sc_register_style!` in the render body.
#[macro_export]
macro_rules! __yew_sc_sync_dom {
    () => {
        $crate::registry::sync_dom()
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
