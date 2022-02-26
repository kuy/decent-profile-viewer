use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Description)]
pub fn title(props: &Props) -> Html {
    html! {
        <div class={css!(r#"
            font-size: 16px;
        "#)}>
            { for props.children.iter() }
        </div>
    }
}
