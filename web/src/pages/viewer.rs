use serde::{Deserialize, Serialize};
use yew_router::{history::Location, prelude::RouterScopeExt};

use crate::components::{Content, Description, Graph, Heading};
use crate::lib::parser::steps;
use crate::lib::profile::analyze;
use crate::lib::profile::{AnalyzedProfile, PROFILES};
use crate::prelude::*;

pub struct ViewerPage {
    profile_name: String,
    profile_notes: String,
    profile_data: AnalyzedProfile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub preset: Option<String>,
    pub visualizer: Option<String>,
}

impl Component for ViewerPage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let location = ctx.link().location().unwrap();
        let query = location.query::<QueryParams>().unwrap();

        let preset_name = query.preset.unwrap_or_else(|| panic!("Unsupported query"));
        let preset = PROFILES
            .iter()
            .find(|preset| preset.name == preset_name)
            .unwrap_or_else(|| panic!("Unknown preset: {}", preset_name));
        let (_, steps) =
            steps(preset.data.as_bytes()).unwrap_or_else(|e| panic!("Parse error: {:?}", e));
        let profile = analyze(&steps);

        Self {
            profile_name: preset.title.clone(),
            profile_notes: preset.notes.clone(),
            profile_data: profile,
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <Page title="Viewer">
                <Heading>{ self.profile_name.as_str() }</Heading>
                <Content>
                    <Graph data={self.profile_data.clone()} />
                    <Description>{ self.profile_notes.clone() }</Description>
                </Content>
            </Page>
        }
    }
}
