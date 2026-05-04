use yew::{
    Callback, Classes, MouseEvent,
    prelude::{Children, Properties},
};

#[derive(Properties, PartialEq)]
pub struct StyledComponentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[derive(Properties, PartialEq)]
pub struct StyledVoidComponentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}
