

![Yew logo](assets/yew_logo.svg) 
# Styled Components for Yew 💅



### Usage:
```rs

styled_component! {
    MyStyledDiv => div {
        border = "solid 1px red";
        padding = 10px;
        color = green;
    }
}

#[component]
fn App() -> Html {
    html! {
        <MyStyledDiv>
            <p>{"Hello Yew-sc!"}</p>
        </MyStyledDiv>
    }
}
```

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
* [ ] Introduce `ToCss` trait for typed values
* [ ] Support primitive values (`px`, `rgb`, etc.)

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
* [ ] `header`, `footer`, `nav`

### Media

* [ ] `img`
* [ ] `video`, `audio`

### Lists

* [ ] `ul`, `ol`, `li`

### Tables

* [ ] `table`, `tr`, `td`, `th`

---

### Void Elements Handling

* [x] Detect void elements:

  * `input`, `img`, `br`, `hr`, `meta`, `link`, etc.
* [x] Generate self-closing tags (`<input />`)
* [ ] Prevent children on void elements (with compile time error)

---

### Attributes Support

### Global Attributes

* [x] `class`
* [ ] `id`
* [ ] `style`
* [ ] `title`
* [ ] `hidden`
* [ ] `tabindex`
* [ ] `role`

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

* [ ] `href`, `target`, `rel`

#### `<img>`

* [ ] `src`, `alt`, `width`, `height`

#### `<input>`

* [ ] `type`, `value`, `placeholder`
* [ ] `checked`, `disabled`, `readonly`

#### `<button>`

* [ ] `type`, `disabled`

#### `<form>`

* [ ] `action`, `method`

---

### DX Improvements

* [ ] Allow ergonomic children (`{ counter.to_string() }`)
* [ ] Improve error messages from macro
* [ ] Provide helper utilities (e.g. `text!()` or `<Text />`)
* [ ] Auto-format style keys (snake_case → kebab-case)

---

### Advanced Features

* [ ] Generate hashed class names (instead of inline styles)
* [ ] Global style registry (deduplicate styles)
* [ ] Extract CSS at compile time
* [ ] SSR compatibility
* [ ] Theming support

---

### Validation & Safety

* [ ] Validate style properties
* [ ] Validate allowed attributes per tag
* [ ] Emit compile-time errors for invalid usage
* [ ] Prevent invalid HTML structures

---

### Architecture

* [ ] Separate macro crate and runtime crate
* [ ] Define shared `ToCss` trait
* [ ] Modular parser + codegen structure

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
