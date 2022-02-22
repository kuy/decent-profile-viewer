use yew::Classes;
use yew_router::components::{Link as YewLink, LinkProps};

use crate::prelude::*;

#[function_component(Link)]
pub fn link(props: &LinkProps<Route>) -> Html {
    html! {
      <YewLink<Route> to={props.to.clone()} classes={styles()}>
        { for props.children.iter() }
      </YewLink<Route>>
    }
}

pub fn styles() -> Classes {
    css!(
        r#"
    color: black;
    text-decoration: none;
    transition: 0.4s;

    &:hover {
        color: #404040;
    }
  "#
    )
    .into()
}
