use crate::components::{Content, Heading};
use crate::prelude::*;

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
                <Content>
                    { "Hello Decent Community!" }
                </Content>
            </Page>
        }
    }
}
