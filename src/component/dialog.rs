use yew::prelude::*;

use crate::component::button::Button;

#[derive(Properties, PartialEq)]
pub struct DialogTitleProps {
    pub children: Html,
}

#[function_component(DialogTitle)]
pub fn dialog_title(props: &DialogTitleProps) -> Html {
    html! {
        <h3 class={classes!("text-lg", "font-bold")}>
            { props.children.clone() }
        </h3>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogContentProps {
    pub children: Html,
}

#[function_component(DialogContent)]
pub fn dialog_content(props: &DialogContentProps) -> Html {
    html! {
        <p class={classes!("py-4")}>
            { props.children.clone() }
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogActionProps {
    pub children: Html,
}

#[function_component(DialogAction)]
pub fn dialog_action(props: &DialogActionProps) -> Html {
    html! {
        <div class={classes!("modal-action")}>
            <form method="dialog">
                { props.children.clone() }
            </form>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DialogProps {
    pub children: Html,
    pub id: String,
}

#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    html! {
        <dialog id={props.id.clone()} class={classes!("modal")}>
            <div class={classes!("modal-box")}>
                { props.children.clone() }
            </div>
        </dialog>
    }
}

#[derive(Properties, PartialEq)]
pub struct ConfirmationDialogProps {
    pub title: String,
    pub message: String,
    pub id: String,
}

#[function_component(ConfirmationDialog)]
pub fn confirmation_dialog(props: &ConfirmationDialogProps) -> Html {
    html! {
        <Dialog id={props.id.clone()}>
            <DialogTitle>
                { props.title.clone() }
            </DialogTitle>
            <DialogContent>
                { props.message.clone() }
            </DialogContent>
            <DialogAction>
                <Button>{"Ok"}</Button>
            </DialogAction>
        </Dialog>
    }
}
