use crate::components::{Content, Heading, PresetItem, PresetList};
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
            <Content>
              <PresetList items={PROFILES
                .iter()
                .map(|preset| {
                  html! { <PresetItem name={preset.name.clone()}>{ preset.title.as_str() }</PresetItem> }
                })
                .collect::<Vec<Html>>()} />
            </Content>
          </Page>
        }
    }
}
