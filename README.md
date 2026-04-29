# Styled Components for Yew 💅

## (WIP)

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
