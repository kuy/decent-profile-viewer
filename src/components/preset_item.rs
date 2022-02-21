use serde::{Deserialize, Serialize};
use yew_router::components::Link as YewLink;

use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,

    #[prop_or_default]
    pub children: Children,
}

#[derive(PartialEq, Clone)]
pub struct PresetItem;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
struct PresetQuery {
    preset: String,
}

impl Component for PresetItem {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let query = Some(PresetQuery {
            preset: ctx.props().name.clone(),
        });
        html! {
            <li class={css!(r#"
          list-style-type: none;
        "#)}>
                <YewLink<Route, PresetQuery> to={Route::Viewer} {query}>
                  { for ctx.props().children.iter() }
                </YewLink<Route, PresetQuery>>
            </li>
        }
    }
}
