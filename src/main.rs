use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

use pages::{AboutPage, NotFoundPage, PresetListPage};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/presets")]
  PresetIndex,
  #[at("/about")]
  About,
  #[not_found]
  #[at("/404")]
  NotFound,
}

struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
  }
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Redirect<Route> to={Route::PresetIndex} /> },
    Route::PresetIndex => html! { <PresetListPage /> },
    Route::About => html! { <AboutPage /> },
    Route::NotFound => html! { <NotFoundPage /> },
  }
}

fn main() {
  yew::start_app::<App>();
}
