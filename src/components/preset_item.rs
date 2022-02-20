use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
}

#[derive(PartialEq, Clone)]
pub struct PresetItem;

impl Component for PresetItem {
  type Message = ();
  type Properties = Props;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
        <li class={css!(r#"
          list-style-type: none;
        "#)}>
            { for ctx.props().children.iter() }
        </li>
    }
  }
}
