use stylist::css;
use yew::html;
use yew::{function_component, Children, Classes, Component, Context, Html, Properties};
use yew_router::components::{Link as LinkBase, LinkProps};

use crate::components::Title;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,

  #[prop_or_default]
  pub title: Option<String>,
}

pub struct Page;

impl Component for Page {
  type Message = ();
  type Properties = Props;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
        <Title text={ctx.props().title.clone()} />
        <header>
          <div class={css!(r#"
            margin: 0 auto;
            width: 1024px;
          "#)}>
            <Link to={Route::Home}>{ "Home" }</Link>
            <Link to={Route::PresetIndex}>{ "Presets" }</Link>
          </div>
        </header>
        <main class="page">
          <div class={css!(r#"
            margin: 0 auto;
            width: 1024px;
          "#)}>
            <h1>{ "Preset List" }</h1>
            { for ctx.props().children.iter() }
          </div>
        </main>
      </>
    }
  }
}

#[function_component(Link)]
fn link(props: &LinkProps<Route>) -> Html {
  html! {
    <LinkBase<Route> to={props.to.clone()} classes={link_style_value()}>
      { for props.children.iter() }
    </LinkBase<Route>>
  }
}

fn link_style_value() -> Classes {
  css!(
    r#"
    color: black;
    text-decoration: none;
  "#
  )
  .into()
}
