<p align="center">
  <img src="example/assets/yew_logo.svg" alt="Yew logo" width="120"/>
</p>

<h1 align="center">yew-sc</h1>

<p align="center"><em>Styled components for <a href="https://yew.rs">Yew</a>, in the spirit of <code>styled-components</code> — but compile-time, type-checked, and zero-runtime-CSS-string-building.</em></p>

<p align="center">
  <a href="#"><img alt="crate" src="https://img.shields.io/badge/crate-yew--sc-orange"></a>
  <a href="LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-blue"></a>
  <a href="#"><img alt="status" src="https://img.shields.io/badge/status-experimental-yellow"></a>
</p>

---

## What it does

`yew-sc` is a procedural macro that turns this:

```rust
use yew::prelude::*;
use yew_sc::styled_component;

styled_component! {
    Card => div {
        border = "solid 3px green";
        background = rgb(100, 65, 87);
        padding = 10px;
    }

    Button => button {
        border = none;
        background = rgb(50, 150, 90);
        color = white;
        padding = 8px;
        cursor = pointer;
        &:hover  { background = rgb(70, 180, 110); }
        &:active { transform = scale(0.97); }
    }
}

#[component]
fn App() -> Html {
    html! {
        <Card>
            <Button>{"Click me"}</Button>
        </Card>
    }
}
```

…into real Yew function components with hashed CSS classes (`ysc-…`) that are
registered once and shared across renders. Style values are validated at
compile time — typos like `colur = red` won't make it past `cargo check`.

A single `styled_component!` invocation accepts any number of
`Name => tag { ... }` entries back-to-back. They share a parse pass but
generate independent components — styles are never merged across siblings.

## Install

```toml
[dependencies]
yew    = { version = "0.23", features = ["csr"] }
yew-sc = "0.1"
```

Run the bundled demo with [`trunk`](https://trunkrs.dev):

```sh
cd example && trunk serve
```

## Features

- **Compile-time CSS validation** — property names are checked against a known
  set; invalid keys are a build error, not a runtime surprise.
- **Compile-time element validation** — only attributes valid for the chosen
  HTML tag are accepted; passing `href` to a `button` won't compile.
- **Hashed class names** — each component compiles to one stable
  `ysc-<hash>` class, registered once on first render.
- **Nested rules** — `&:hover`, `&:active`, etc. via the `&:` prefix.
- **Dynamic values without re-registration** — `$name` and `${ expr }` become
  CSS custom properties; the stylesheet stays static.
- **Bring your own props struct** — opt into full control over the component's
  prop surface with `#[styled_props]`.
- **Sensible defaults** — `class`, `children`, `onclick`, `id`, `title`,
  `hidden`, `tabindex`, `role` are forwarded automatically.

## Dynamic styles

Reference a dynamic value with `$name` inside the style block and the macro
generates a matching prop on the component. At render time, the prop value is
injected as a CSS custom property on the element's `style` attribute. The CSS
itself remains a single static hashed class — there is **no** per-render style
registration.

```rust
styled_component! {
    Button => button {
        background = $bg;
        color = $fg;
        padding = 8px;
        &:hover { background = $bg_hover; }
    }
}

#[component]
fn App() -> Html {
    html! {
        <Button bg="rebeccapurple" fg="white" bg_hover="indigo">
            {"click"}
        </Button>
    }
}
```

What gets emitted:

- In CSS, `$bg` is rewritten to `var(--bg)`, so the stylesheet contains
  `background: var(--bg);` and `background: var(--bg-hover);` inside `:hover`.
- On the element: `style="--bg: rebeccapurple; --fg: white; --bg-hover: indigo;"`.

Rust identifier underscores become dashes in the emitted CSS, so the Rust prop
`bg_hover` corresponds to the CSS variable `--bg-hover`.

### Inline Rust expressions: `${ expr }`

For conditional or computed values, embed any Rust expression that evaluates
to a `ToCssVar` value:

```rust
styled_component! {
    MyButton<MyButtonProps> => button {
        background = ${ if props.is_danger { "crimson" } else { "steelblue" } };
        opacity    = ${ if props.disabled  { 0.5 }      else { 1.0 } };
    }
}
```

Each `${ ... }` is lowered to a synthetic CSS custom property (e.g.
`var(--__yew-sc-expr-0)`) and evaluated once per render to set its value on the
element's `style` attribute. The stylesheet stays static — only the inline
custom-property values change per render.

Notes:

- The expression body is real Rust: `if` arms need braces and matching
  types (use `String::from(...)` or a common return type if arms differ).
- Anything the expression references must be in scope at the generated
  component's call site — `props.<field>` is the common case.
- The expression is evaluated inside an `if let Some(...) = ToCssVar::...`
  guard, so returning `None` (e.g. from an `Option<T>` branch) **omits** that
  variable for that render.

## Bring-your-own props struct

For full control — including transient, styling-only props that don't map to
HTML attributes — provide a props type between angle brackets. Annotate the
struct with `#[styled_props]` to inject the base fields (`class`, `onclick`,
`children`, `id`, `title`, `hidden`, `tabindex`, `role`) automatically:

```rust
use yew_sc::{styled_component, styled_props};

#[styled_props]
pub struct MyButtonProps {
    #[prop_or_default]
    pub is_danger: bool,
}

styled_component! {
    MyButton<MyButtonProps> => button {
        background = ${ if props.is_danger { "crimson" } else { "steelblue" } };
        color      = white;
    }
}

html! { <MyButton is_danger=true>{"danger"}</MyButton> }
```

`#[styled_props]` adds the base fields if you haven't declared them, then
derives `Properties` and `PartialEq` (skipped if your struct already has
either). You only write the fields the macro doesn't already know about.

Rules:

- The macro does **not** add tag-specific attrs (`href`, `etype`, `disabled`,
  …) when you bring your own struct. Declare them yourself and forward them
  through your render path — this mode hands the macro responsibility back to
  you.
- Every `$name` referenced in the style block must exist as a field on your
  struct, and its type must implement `ToCssVar`. Built-in impls cover
  `bool`, numeric types, `String`, `&str`, `AttrValue`, and `Option<T>` where
  `T: ToCssVar`. `Option::None` skips that variable entirely.
- Yew's `html!` does not understand `$` at call sites — pass transient props
  as normal Yew props (`is_danger=true`). The `$` prefix lives only inside
  the `styled_component!` style block.

## Animations

Declare `@keyframes` alongside your components with the `keyframes` keyword.
Stops accept `from` / `to` / percent literals (`0%`, `50%`, `100%`). The macro
hashes each keyframes block and rewrites references inside `animation` and
`animation-name` values, so two components can declare a `spin` without
colliding.

```rust
styled_component! {
    keyframes spin {
        from { transform = rotate(0deg); }
        to   { transform = rotate(360deg); }
    }

    keyframes pulse {
        0%   { opacity = 0.4; }
        50%  { opacity = 1.0; }
        100% { opacity = 0.4; }
    }

    Spinner => div {
        width = 24px;
        height = 24px;
        border = "3px solid rgba(168,179,255,0.25)";
        border_top = "3px solid #a8b3ff";
        border_radius = 999px;
        animation_name = spin;
        animation_duration = 800ms;
        animation_timing_function = linear;
        animation_iteration_count = "infinite";
    }

    // shorthand — the name is rewritten inside the string literal too
    Pulse => span {
        display = "inline-block";
        animation = "pulse 1.6s ease-in-out infinite";
    }
}
```

What gets emitted:

- `spin` is hashed to e.g. `spin-7aabb432687ba56b`, registered once as
  `@keyframes spin-7aabb432687ba56b { from { … } to { … } }`.
- `animation-name: spin;` becomes `animation-name: spin-7aabb432687ba56b;`.
- Inside the `animation` shorthand string, the name is replaced with
  word-boundary matching — `"pulse 1.6s …"` → `"pulse-832a1a103cb3ae34 1.6s …"`.

Notes:

- `keyframes` declarations live at the top level of `styled_component!`,
  next to your component declarations — they're shared across every component
  in the same macro invocation.
- The shorthand `animation = spin 1s linear infinite;` (bare tokens) won't
  parse — use a string literal: `animation = "spin 1s linear infinite";`. Or
  use the longhand `animation_name = spin;` plus the other `animation_*`
  properties.
- Timing functions `cubic_bezier(...)` and `steps(...)` are accepted as
  CSS functions.

## Supported HTML

| Category    | Tags                                                       |
|-------------|------------------------------------------------------------|
| Text        | `div`, `span`, `p`, `h1`–`h6`                              |
| Form        | `button`, `input`, `textarea`, `form`                      |
| Structural  | `section`, `article`, `main`, `header`, `footer`, `nav`    |
| Media       | `img`, `video`, `audio`                                    |
| Lists       | `ul`, `ol`, `li`                                           |
| Tables      | `table`, `tr`, `td`, `th`                                  |

Void elements (`input`, `img`, `br`, `hr`, `meta`, `link`, …) render
self-closing and refuse children at compile time.

### Element-specific attributes

- `<a>` — `href`, `target`, `rel`
- `<img>` — `src`, `alt`, `width`, `height`
- `<input>` — `etype`, `value`, `placeholder`, `checked`, `disabled`, `readonly`
- `<button>` — `etype`, `disabled`
- `<form>` — `action`, `method`

### A note on `etype`

The HTML `type` attribute is exposed on `<input>` and `<button>` as the prop
`etype` (short for *element type*). `type` is a reserved keyword in Rust, so it
cannot be used as a struct field name without `r#type`. The macro maps `etype`
back to `type` in the generated HTML, so the rendered markup is unchanged:

```rust
html! {
    <NameInput etype="text" placeholder="your name"/>
    // renders: <input type="text" placeholder="your name" class="ysc-..."/>
}
```

## Workspace layout

```
yew-sc/
├── src/             # facade crate that re-exports the macro + runtime helpers
├── yew-sc-core/     # runtime: style registry, ToCssVar trait, helpers
├── yew-sc-macros/   # proc-macros: parser + codegen for `styled_component!`
└── example/         # Yew app demonstrating the macro
```

## Status

Experimental — the API may break before `0.2`. Feedback, issues, and PRs are
welcome.

## License

MIT
