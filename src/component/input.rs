use gloo::file::File;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileInputProps {
    #[prop_or_default]
    pub multiple: bool,

    pub id: String,

    #[prop_or("Load File".to_string())]
    pub prompt: String,

    #[prop_or("Choose File".to_string())]
    pub placeholder: String,

    pub on_change: Callback<File>,
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let file_name_input_id = props.id.clone() + "-filename";
    let on_change_clone = props.on_change.clone();
    let onchange = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let file = input.files().unwrap().get(0).map(File::from).unwrap();
        let file_name = file.name();
        let file_name_input = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(file_name_input_id.as_str())
            .unwrap()
            .unchecked_into::<HtmlInputElement>();
        file_name_input.set_value(&file_name);
        on_change_clone.emit(file);
    });

    let id_clone = props.id.clone();
    let onclick = Callback::from(move |_: MouseEvent| {
        let file_input_element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id_clone.as_str())
            .unwrap()
            .unchecked_into::<HtmlInputElement>();
        file_input_element.click();
    });

    html! {
        <>
            <input
                id={props.id.clone()}
                class={classes!("hidden")}
                type="file"
                multiple={props.multiple}
                {onchange}/>
            <label for={props.id.clone()}>
                <div class={classes!("join")}>
                    <input
                        id={props.id.clone() + "-filename"}
                        class={classes!("input", "input-bordered", "input-primary", "join-item")}
                        placeholder={props.placeholder.clone()}
                        disabled={true}/>
                    <button
                        id={props.id.clone() + "-button"}
                        class={classes!("btn", "btn-primary", "join-item")}
                        {onclick}>
                        {props.prompt.clone()}
                    </button>
                </div>
            </label>
        </>
    }
}
