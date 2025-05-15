use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub children: Html,
}

#[function_component(Navbar)]
pub fn nabvar(props: &NavbarProps) -> Html {
    html! {
        <nav class={classes!("navbar", "bg-base-300")}>
            { props.children.clone() }
        </nav>
    }
}
