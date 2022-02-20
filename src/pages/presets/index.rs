use crate::components::{Heading, Page, PresetItem, PresetList};
use crate::lib::profile::PROFILES;
use crate::prelude::*;

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
        <ul>
          { PROFILES.iter().map(|(_, preset)| {
            html!{ <li>{ format!("{}", &preset.title) }</li> }
          }).collect::<Html>() }
        </ul>
      </Page>
    }
  }
}
