use yew::prelude::*;

use super::{button::Button, r#box::Box};

#[derive(Properties, PartialEq)]
struct PageChangeButtonProps {
    pub children: Html,

    #[prop_or(false)]
    pub enabled: bool,

    #[prop_or(Callback::from(|_| {}))]
    pub on_click: Callback<MouseEvent>,
}

#[function_component(PageChangeButton)]
fn page_change_button(props: &PageChangeButtonProps) -> Html {
    if props.enabled {
        html! {
            <Button class={classes!("join-item")} onclick={props.on_click.clone()}>
                { props.children.clone() }
            </Button>
        }
    } else {
        html! {
            <Button class={classes!("join-item", "btn-disabled")} onclick={props.on_click.clone()}>
                { props.children.clone() }
            </Button>
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    #[prop_or(0usize)]
    pub count: usize,

    #[prop_or(Callback::from(move |_:usize| {}))]
    pub on_change: Callback<usize>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let current_page_state = use_state(|| 0usize);

    let increment = {
        let current_page_state = current_page_state.clone();
        let count = props.count;
        let on_change = props.on_change.clone();
        Callback::from(move |_e: MouseEvent| {
            if count != 0 && *current_page_state < count {
                current_page_state.set(*current_page_state + 1);
                on_change.emit(*current_page_state + 1);
            }
        })
    };

    let decrement = {
        let current_page_state = current_page_state.clone();
        let count = props.count;
        let on_change = props.on_change.clone();
        Callback::from(move |_e: MouseEvent| {
            if count != 0 && *current_page_state > 0 {
                current_page_state.set(*current_page_state - 1);
                on_change.emit(*current_page_state - 1);
            }
        })
    };

    let last_page = if props.count == 0 { 0 } else { props.count - 1 };
    let current_page = if props.count == 0 {
        0
    } else {
        *current_page_state + 1
    };
    html! {
        <Box class={classes!("join", "flex", "justify-center")}>
            <PageChangeButton enabled={*current_page_state > 0} on_click={decrement}>
                {"«"}
            </PageChangeButton>
            <Box class={
                classes!("join-item", "h-12", "min-h-12", "text-sm", "font-semibold", "items-center", "justify-center", "inline-flex", "grow")
                }>
                {

                    format!(
                        "Page {}/{}",
                        current_page,
                        props.count)
                }
            </Box>
            <PageChangeButton enabled={*current_page_state < last_page } on_click={increment}>
                {"»"}
            </PageChangeButton>
        </Box>
    }
}
