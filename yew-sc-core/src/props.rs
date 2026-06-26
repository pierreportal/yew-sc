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

#[derive(Properties, PartialEq)]
pub struct StyledAnchorProps {
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
    #[prop_or_default]
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub target: Option<AttrValue>,
    #[prop_or_default]
    pub rel: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct StyledButtonProps {
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
    #[prop_or_default]
    pub etype: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
}

#[derive(Properties, PartialEq)]
pub struct StyledFormProps {
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
    #[prop_or_default]
    pub action: Option<AttrValue>,
    #[prop_or_default]
    pub method: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct StyledImgProps {
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
    #[prop_or_default]
    pub src: Option<AttrValue>,
    #[prop_or_default]
    pub alt: Option<AttrValue>,
    #[prop_or_default]
    pub width: Option<AttrValue>,
    #[prop_or_default]
    pub height: Option<AttrValue>,
}

#[derive(Properties, PartialEq)]
pub struct StyledInputProps {
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
    #[prop_or_default]
    pub etype: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
}
