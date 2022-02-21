use crate::prelude::*;

pub struct NotFoundPage;

impl Component for NotFoundPage {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <Page>
                <div>{ "404 | Not Found" }</div>
            </Page>
        }
    }
}
