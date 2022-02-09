use yew::html;
use yew::{html::ChildrenWithProps, Component, Context, Html, Properties};

use crate::components::PresetItem;

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
