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

    for (class, css) in entries {
        let css_full = format!(".{} {{ {} }}", class, css);
        output.push_str(&css_full);
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
    register(&a, &b);
    inject_style();
}
