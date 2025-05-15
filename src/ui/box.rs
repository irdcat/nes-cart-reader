use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BoxProps {
    pub children: Html,

    #[prop_or(classes!())]
    pub class: Classes,
}

#[function_component(Box)]
pub fn r#box(props: &BoxProps) -> Html {
    html! {
        <div class={props.class.clone()}>
            { props.children.clone() }
        </div>
    }
}
