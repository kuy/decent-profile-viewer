use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod lib;
mod pages;
mod prelude;
mod routes;

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
        <Switch<routes::Route> render={Switch::render(routes::switch)} />
      </BrowserRouter>
    }
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<App>();
}
