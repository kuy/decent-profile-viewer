use yew::ChildrenWithProps;

use crate::components::PresetItem;
use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: ChildrenWithProps<PresetItem>,
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
        <ul>
            { for ctx.props().children.iter() }
        </ul>
    }
  }
}
