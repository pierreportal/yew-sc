use yew::{
    AttrValue, Callback, Classes, MouseEvent,
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
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub tabindex: Option<AttrValue>,
    #[prop_or_default]
    pub role: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct StyledVoidComponentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub tabindex: Option<AttrValue>,
    #[prop_or_default]
    pub role: Option<AttrValue>,
}
