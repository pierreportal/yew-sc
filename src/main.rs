use yew::prelude::*;
use yew_sc_core::{props::StyledComponentProps, registry::register_style};
use yew_sc_macros::styled_component;

styled_component! {
    StyledDiv => div {
        border = "solid 3px green";
        background = rgb(100,65,87);
        padding = 10px;
    }
}
styled_component! {
    Title => h1 {
        color = red;
    }
}
styled_component! {
    Button => button {
        border = none;
    }
}

#[component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let handle_click = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <StyledDiv>
            <Title>{"Hey!"}</Title>
            <Button onclick={handle_click}><>{*counter}</></Button>
        </StyledDiv>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
