use yew::{create_portal, function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct TitleProps {
  #[prop_or_default]
  pub children: Children,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
  let content_host = gloo_utils::document()
    .query_selector("head > title")
    .expect("should exist")
    .unwrap();

  create_portal(html! { { for props.children.iter() } }, content_host.into())
}
