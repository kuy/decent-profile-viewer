use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
        <div class={css!(r#"
            margin-top: 8px;
        "#)}>
            { for props.children.iter() }
        </div>
    }
}
