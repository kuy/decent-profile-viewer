use serde::{Deserialize, Serialize};
use yew_router::{history::Location, prelude::RouterScopeExt};

use crate::prelude::*;

pub struct ViewerPage;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
  pub preset: Option<String>,
  pub visualizer: Option<String>,
}

impl Component for ViewerPage {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let location = ctx.link().location().unwrap();
    let query = location.query::<QueryParams>().unwrap();
    log::info!("{:?}", query);
    html! {
      <Page title="Viewer">
        { "Viewer Here" }
      </Page>
    }
  }
}
