

![Yew logo](example/assets/yew_logo.svg) 
# Styled Components for Yew 💅



### Usage:
```rs
use yew::prelude::*;
use yew_sc::styled_component;

styled_component! {
    StyledDiv => div {
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
        &:hover {
            background = rgb(70, 180, 110);
        }
        &:active {
            transform = scale(0.97);
        }
    }
}

#[component]
fn App() -> Html {
    html! {
        <StyledDiv>
            <Button>{"Click me"}</Button>
        </StyledDiv>
    }
}
```

A single `styled_component!` invocation accepts any number of `Name => tag { ... }`
entries back-to-back. They share a parse pass but generate independent
components — no styles are merged across siblings.

## Workspace Layout

```
yew-sc/
├── yew-sc-core/    # runtime: style registry, helpers
├── yew-sc-macros/  # `styled_component!` macro: parser + codegen
└── example/        # Yew app demonstrating the macro
```

Run the example with `trunk serve` from `example/`.

### Dynamic styles

Reference a dynamic value with `$name` inside the style block and the macro
will generate a matching prop on the component. At render time the prop value
is injected as a CSS custom property on the element's `style` attribute. The
CSS itself stays as one static hashed class — there is no per-render style
registration.

```rs
styled_component! {
    Button => button {
        background = $bg;
        color = $fg;
        padding = 8px;
        &:hover {
            background = $bg_hover;
        }
    }
}

#[component]
fn App() -> Html {
    html! {
        <Button bg="rebeccapurple" fg="white" bg_hover="indigo">{"click"}</Button>
    }
}
```

What gets emitted:

- In CSS: `$bg` is rewritten to `var(--bg)`. So the generated stylesheet contains
  `background: var(--bg);` and `background: var(--bg-hover);` inside `:hover`.
- On the element: `style="--bg: rebeccapurple; --fg: white; --bg-hover: indigo;"`.

Rust identifier underscores become dashes in the emitted CSS, so the Rust prop
`bg_hover` corresponds to the CSS variable `--bg-hover`.

#### Inline Rust expressions: `${ expr }`

For conditional or computed values, embed any Rust expression that evaluates
to a `ToCssVar` value:

```rs
styled_component! {
    MyButton<MyButtonProps> => button {
        background = ${ if props.is_danger { "crimson" } else { "steelblue" } };
        opacity = ${ if props.disabled { 0.5 } else { 1.0 } };
    }
}
```

The macro lowers each `${ ... }` to a synthetic CSS custom property (e.g.
`var(--__yew-sc-expr-0)`) and evaluates the expression once per render to set
its value on the element's `style` attribute. The stylesheet itself stays
static — only the inline custom-property values change per render.

Notes:

- The expression body is real Rust, so `if` arms need braces and matching
  types (use `String::from(...)` or a common return type if the arms differ).
- Anything the expression references must be in scope at the call site of the
  generated component — `props.<field>` is the common case.
- The expression evaluates lazily inside an `if let Some(...) = ToCssVar::...`
  guard, so returning `None` (e.g. from an `Option<T>` branch) omits the
  variable for that render.

#### Bring-your-own props struct

For full control over the component's props — including transient
styling-only props that don't map to HTML attributes — provide a props type
between angle brackets. Annotate the struct with `#[styled_props]` to inject
the base fields (`class`, `onclick`, `children`, `id`, `title`, `hidden`,
`tabindex`, `role`) automatically:

```rs
use yew_sc::{styled_component, styled_props};

#[styled_props]
pub struct MyButtonProps {
    #[prop_or_default]
    pub is_danger: bool,
}

styled_component! {
    MyButton<MyButtonProps> => button {
        background = ${ if props.is_danger { "crimson" } else { "steelblue" } };
        color = white;
    }
}

html! {
    <MyButton is_danger=true>{"danger"}</MyButton>
}
```

`#[styled_props]` adds the base fields if you haven't declared them, then
derives `Properties` and `PartialEq` (skips the derive if your struct already
has it). You only write fields the macro doesn't know about — typically the
transient styling props plus any element-specific attrs you want forwarded.

Rules for user-provided props structs:

- The macro does **not** add the tag's element-specific attrs (`href`, `etype`,
  `disabled`, etc.). If you want them forwarded, declare them yourself and
  thread them through your own render path — this mode hands the macro
  responsibility back to you.
- Every `$name` referenced in the style block must exist as a field on your
  struct, and its type must implement `ToCssVar`. Built-in impls cover
  `bool`, numeric types, `String`, `&str`, `AttrValue`, and `Option<T>` where
  `T: ToCssVar`. `Option::None` skips that variable entirely.
- Yew's `html!` does not understand `$` at call sites — pass transient props
  as normal Yew props (`is_danger=true`). The `$` prefix lives only inside
  the `styled_component!` style block.

## TODO

### Core

* [x] Implement `styled_component!` parsing (tag, name, styles)
* [x] Generate Yew `function_component`
* [x] Support children (`Html`)
* [x] Merge user `class` with generated styles
* [x] Support event handlers (`onclick`, etc.)

---

### Styling System

* [x] Parse key/value style pairs
* [x] Convert styles to inline CSS string
* [x] Support primitive values (`px`, `rgb`, keywords, etc.)
* [x] Compile-time validation of CSS property names
* [x] Nested rules with `&:` syntax (e.g. `&:hover`, `&:active`)
* [x] Introduce `ToCss` trait for typed values

---

### HTML Tag Support

### Core Elements

* [x] `div`, `span`, `p`
* [x] `h1` → `h6`
* [x] `button`
* [x] `input`
* [x] `textarea`

### Layout / Structure

* [x] `section`, `article`, `main`
* [x] `header`, `footer`, `nav`

### Media

* [x] `img`
* [x] `video`, `audio`

### Lists

* [x] `ul`, `ol`, `li`

### Tables

* [x] `table`, `tr`, `td`, `th`

---

### Void Elements Handling

* [x] Detect void elements:

  * `input`, `img`, `br`, `hr`, `meta`, `link`, etc.
* [x] Generate self-closing tags (`<input />`)
* [x] Prevent children on void elements (with compile time error)

---

### Attributes Support

### Global Attributes

* [x] `class`
* [x] `id`
* [ ] `style`
* [x] `title`
* [x] `hidden`
* [x] `tabindex`
* [x] `role`

### Events (Yew Callbacks)

* [x] `onclick`
* [ ] `oninput`
* [ ] `onchange`
* [ ] `onblur`
* [ ] `onfocus`
* [ ] `onkeydown`
* [ ] `onkeyup`

### Element-Specific

#### `<a>`

* [x] `href`, `target`, `rel`

#### `<img>`

* [x] `src`, `alt`, `width`, `height`

#### `<input>`

* [x] `type` (use `etype` — see note below), `value`, `placeholder`
* [x] `checked`, `disabled`, `readonly`

#### `<button>`

* [x] `type` (use `etype` — see note below), `disabled`

#### Note on `etype`

The HTML `type` attribute is exposed on `<input>` and `<button>` as the prop
`etype` (short for *element type*). `type` is a reserved keyword in Rust, so it
cannot be used as a struct field name without the awkward raw-identifier
escape `r#type`. The macro maps `etype` back to `type` in the generated HTML,
so the rendered markup is unchanged:

```rs
html! {
    <NameInput etype="text" placeholder="your name"/>
    // renders: <input type="text" placeholder="your name" class="ysc-..."/>
}
```

#### `<form>`

* [x] `action`, `method`

---

### DX Improvements

* [ ] Allow ergonomic children (`{ counter.to_string() }`)
* [ ] Improve error messages from macro
* [ ] Provide helper utilities (e.g. `text!()` or `<Text />`)
* [ ] Auto-format style keys (snake_case → kebab-case)

---

### Advanced Features

* [x] Generate hashed class names (instead of inline styles)
* [x] Global style registry (deduplicate styles)
* [ ] Extract CSS at compile time
* [ ] SSR compatibility
* [ ] Theming support

---

### Validation & Safety

* [x] Validate style properties (compile-time check against known CSS properties)
* [x] Validate allowed attributes per tag
* [x] Emit compile-time errors for invalid usage
* [x] Prevent invalid HTML structures

---

### Architecture

* [x] Separate macro crate (`yew-sc-macros`) and runtime crate (`yew-sc-core`)
* [x] Modular parser + codegen structure
* [x] Define shared `ToCss` trait

---

### Documentation

* [ ] Basic usage examples
* [ ] Styled component patterns
* [ ] Integration with Yew apps
* [ ] Comparison with React styled-components

---

### Future Ideas

* [ ] Custom DSL for styles (e.g. `padding: 10px`)
* [ ] Animation support
* [ ] Devtools / debug output
* [ ] Plugin system for extensions
