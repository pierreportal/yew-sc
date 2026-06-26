use yew::prelude::*;
use yew_sc::{styled_component, styled_props};

#[styled_props]
pub struct PillProps {
    #[prop_or_default]
    pub is_danger: bool,
}

styled_component! {
    Page => main {
        display = flex;
        flex_direction = column;
        align_items = center;
        gap = 32px;
        padding = 48px;
        min_height = 100vh;
        box_sizing = "border-box";
    }

    Card => section {
        display = flex;
        flex_direction = column;
        gap = 20px;
        background = "rgba(255,255,255,0.04)";
        border = "1px solid rgba(255,255,255,0.08)";
        border_radius = 16px;
        padding = 32px;
        max_width = 480px;
        width = "100%";
        box_shadow = "0 20px 60px rgba(0,0,0,0.35)";
    }

    Title => h1 {
        margin = 0;
        font_size = 28px;
        font_weight = 600;
        letter_spacing = "-0.02em";
        color = "#a8b3ff";
    }

    Subtitle => p {
        margin = 0;
        color = "rgba(241,243,248,0.6)";
        font_size = 14px;
        line_height = 1.6;
    }

    Row => div {
        display = flex;
        gap = 12px;
        flex_wrap = wrap;
        align_items = center;
    }

    NameInput => input {
        outline = none;
        flex = 1;
        min_width = 200px;
        background = "rgba(0,0,0,0.25)";
        border = "1px solid rgba(255,255,255,0.12)";
        border_radius = 10px;
        padding = "12px 14px";
        color = "#f1f3f8";
        font_size = 14px;
        transition = "border-color 120ms ease";

        &:hover {
            border_color = "rgba(255,255,255,0.24)";
        }
        &:focus {
            border_color = "#a8b3ff";
        }
    }

    // $name interpolation drives CSS custom properties from props.
    Counter => button {
        border = none;
        background = $bg;
        color = $fg;
        font_size = 16px;
        font_weight = 600;
        padding = "12px 20px";
        border_radius = 10px;
        cursor = pointer;
        transition = "transform 120ms ease, filter 120ms ease";
        &:hover {
            transform = scale(1.02);
        }
        &:active {
            transform = scale(0.97);
        }
    }

    // Bring-your-own props + inline ${} expressions.
    Pill<PillProps> => span {
        display = "inline-block";
        padding = "4px 12px";
        border_radius = 999px;
        font_size = 12px;
        font_weight = 600;
        text_transform = uppercase;
        letter_spacing = "0.08em";
        background = ${ if props.is_danger { "rgba(244,63,94,0.18)" } else { "rgba(52,211,153,0.18)" } };
        color = ${ if props.is_danger { "#fda4af" } else { "#6ee7b7" } };
        border = ${ if props.is_danger { "1px solid rgba(244,63,94,0.35)" } else { "1px solid rgba(52,211,153,0.35)" } };
    }

    // Element-specific attribute validation (only <a> accepts href).
    Link => a {
        color = "#a8b3ff";
        text_decoration = none;
        font_size = 13px;
        &:hover {
            text_decoration = underline;
        }
    }
}

#[component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let inc = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let dec = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    html! {
        <Page>
            <Card>
                <Title>{"yew-sc"}</Title>
                <Subtitle>
                    {"Styled components for Yew. Compile-time CSS validation, "}
                    {"per-component props, and dynamic styles via CSS variables."}
                </Subtitle>

                <Row>
                    <NameInput
                        etype="text"
                        placeholder="Your name"
                        tabindex="0"
                    />
                    <Counter bg="#a8b3ff" fg="#0a0f1f" onclick={inc}>
                        <>{format!("+ {}", *counter)}</>
                    </Counter>
                    <Counter bg="rgba(255,255,255,0.08)" fg="#f1f3f8" onclick={dec}>{"−"}</Counter>
                </Row>

                <Row>
                    <Pill is_danger=false>{"styled"}</Pill>
                    <Pill is_danger=true>{"dynamic"}</Pill>
                    <Pill is_danger=false>{"validated"}</Pill>
                </Row>

                <Link href="https://github.com/pierreportal/yew-sc" target="_blank" rel="noopener noreferrer">
                    {"View on GitHub →"}
                </Link>
            </Card>
        </Page>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
