use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub items: Vec<Html>,
}

pub struct PresetList;

impl Component for PresetList {
  type Message = ();
  type Properties = Props;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
        <ul class={css!(r#"
          padding-inline: 0;
        "#)}>
            { ctx.props().items.clone() }
        </ul>
    }
  }
}
