use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{AboutPage, NotFoundPage, PresetListPage, ViewerPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/viewer")]
  Viewer,
  #[at("/presets")]
  PresetIndex,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Redirect<Route> to={Route::PresetIndex} /> },
    Route::Viewer => html! { <ViewerPage /> },
    Route::PresetIndex => html! { <PresetListPage /> },
    Route::About => html! { <AboutPage /> },
    Route::NotFound => html! { <NotFoundPage /> },
  }
}
