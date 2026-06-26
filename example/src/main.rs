use yew::prelude::*;
use yew_sc::styled_component;

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
        background = rgb(50,150,90);
        color = white;
        padding = 8px;
        cursor = pointer;
        &:hover {
            background = rgb(70,180,110);
        }
        &:active {
            transform = scale(0.97);
        }
    }
}
styled_component! {
    NameInput => input {
        outline = none;
        border = "dashed 2px rgb(0,0,255)";
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
            <NameInput/>
            <Button onclick={handle_click}><>{*counter}</></Button>
        </StyledDiv>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
