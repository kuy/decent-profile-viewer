use yew::{html, Component, Context, Html};

use crate::components::{Heading, Page};

pub struct AboutPage;

impl Component for AboutPage {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <Page title="About">
        <Heading>{ "About" }</Heading>
        { "Hello Decent Community." }
      </Page>
    }
  }
}
