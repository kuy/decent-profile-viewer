use yew::{html, Component, Context, Html};

use crate::components::{Heading, Page, PresetItem, PresetList};

pub struct PresetListPage;

impl Component for PresetListPage {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <Page title="Presets">
        <Heading>{ "Presets" }</Heading>
        <PresetList>
          <PresetItem>{ "1" }</PresetItem>
          <PresetItem>{ "2" }</PresetItem>
          <PresetItem>{ "3" }</PresetItem>
        </PresetList>
      </Page>
    }
  }
}
