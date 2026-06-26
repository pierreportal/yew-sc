

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
}

styled_component! {
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

## Workspace Layout

```
yew-sc/
├── yew-sc-core/    # runtime: style registry, helpers
├── yew-sc-macros/  # `styled_component!` macro: parser + codegen
├── example/        # Yew app demonstrating the macro
└── xtask/          # workspace tasks
```

Run the example with `trunk serve` from `example/`.

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
    // renders: <input type="text" placeholder="your name" class="sc-..."/>
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
