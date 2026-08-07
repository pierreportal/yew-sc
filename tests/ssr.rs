//! Only compiled when built with `--no-default-features --features ssr`
//! (see the `ssr-build` CI job) — the default `csr` feature pulls in
//! browser-only dependencies that don't exist in this test's target.
#![cfg(feature = "ssr")]

use yew::prelude::*;
use yew_sc::styled_component;

styled_component! {
    Card => div {
        color = red;
        padding = 10px;
    }
}

#[function_component]
fn App() -> Html {
    html! { <Card>{"hi"}</Card> }
}

#[tokio::test(flavor = "current_thread")]
async fn ssr_render_populates_the_stylesheet() {
    let renderer = yew::ServerRenderer::<App>::new();
    let html = renderer.render().await;

    assert!(
        html.contains("class=\"ysc-"),
        "expected a hashed class in the rendered HTML, got: {html}"
    );

    let sheet = yew_sc::stylesheet();
    assert!(
        sheet.contains("color: red") && sheet.contains("padding: 10px"),
        "expected the component's CSS in the stylesheet, got: {sheet}"
    );
}
