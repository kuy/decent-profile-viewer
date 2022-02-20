use yew_router::components::Link;

use crate::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub to: Route,
}

#[derive(PartialEq)]
pub struct Logo;

impl Component for Logo {
  type Message = ();
  type Properties = Props;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
        <Link<Route> to={ctx.props().to.clone()} classes="logo">
            <img src={ "/logo.png" } class={css!(r#"
                width: 64px;
                height: 64px;

                transition: 0.4s;

                &:hover {
                    opacity: 0.8;
                }
            "#)} />
        </Link<Route>>
    }
  }
}
