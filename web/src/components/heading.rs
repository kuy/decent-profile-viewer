use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[derive(PartialEq)]
pub struct Heading;

impl Component for Heading {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class={css!(r#"
                font-weight: 400;
                font-size: 24px;
            "#)}>
                { for ctx.props().children.iter() }
            </h1>
        }
    }
}
