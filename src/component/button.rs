use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Html,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button class={classes!("btn")}>
            { props.children.clone() }
        </button>
    }
}
