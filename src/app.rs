use std::collections::HashMap;
use wasm_bindgen::JsCast;

use gloo::file::{
    callbacks::{read_as_bytes, FileReader},
    File,
};
use uuid::Uuid;
use web_sys::{HtmlButtonElement, HtmlDialogElement, HtmlInputElement};
use yew::{classes, html, Callback, Component, Context, Event, Html, TargetCast};

use crate::header::Header;
use crate::rom::reader::{RomReader, RomReaderParams, RomReaderResult};

pub struct App {
    readers: HashMap<String, FileReader>,
    result: Option<RomReaderResult>,
    error: String,
}

pub enum AppMessage {
    Uploaded(File),
    LoadSuccess(String, Vec<u8>),
    LoadFailure(String, String),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::with_capacity(1),
            result: None,
            error: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::Uploaded(file) => {
                let link = ctx.link().clone();
                let uuid = Uuid::new_v4().to_string();
                let task = {
                    let uuid = uuid.clone();
                    read_as_bytes(&file, move |result| {
                        let msg = match result {
                            Ok(data) => AppMessage::LoadSuccess(uuid, data),
                            Err(err) => AppMessage::LoadFailure(uuid, err.to_string()),
                        };
                        link.send_message(msg);
                    })
                };
                self.readers.insert(uuid, task);
                true
            }
            AppMessage::LoadSuccess(uuid, bytes) => {
                let link = ctx.link().clone();
                match RomReader::read(RomReaderParams {
                    data: bytes,
                    origin: 0xC000,
                }) {
                    Ok(result) => {
                        self.result = Some(result);
                        self.readers.remove(&uuid);
                        true
                    }
                    Err(error) => {
                        link.send_message(AppMessage::LoadFailure(uuid, error.to_string()));
                        false
                    }
                }
            }
            AppMessage::LoadFailure(uuid, message) => {
                self.result = None;
                self.error = message;
                self.readers.remove(&uuid);
                let dialog: HtmlDialogElement = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id("romLoadDialog")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                dialog.show_modal().unwrap();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().unwrap().get(0).map(File::from).unwrap();
            let file_name = file.name();
            let input_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("romName")
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            input_element.set_value(&file_name);
            AppMessage::Uploaded(file)
        });

        let onclick = Callback::from(move |_| {
            let button_element = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("romInput")
                .unwrap()
                .unchecked_into::<HtmlButtonElement>();
            button_element.click();
        });

        let error_message = self.error.clone();

        html! {
            <>
                <div class={classes!("flex")}>
                    <div class={classes!("grow")}>
                        <nav class={classes!("navbar", "bg-base-100")}>
                            <input id="romInput" type="file" multiple={false} {onchange} class={classes!("hidden")}/>
                            <label for="romInput">
                                <div class={classes!("join")}>
                                    <input id="romName" class={classes!("input", "input-bordered", "join-item")} placeholder="Choose ROM"/>
                                    <button class={classes!("btn", "join-item")} {onclick}>{"Load ROM"}</button>
                                </div>
                            </label>
                        </nav>
                    </div>
                    <div classes={classes!("grow-0")}>
                        <main>
                            <Header rom_header={ self.result.as_ref().map(|v| v.header) }/>
                        </main>
                    </div>
                </div>
                <dialog id="romLoadDialog" class={classes!("modal")}>
                    <div class={classes!("modal-box")}>
                        <h3 class={classes!("text-lg", "font-bold")}>{"Error"}</h3>
                        <p class={classes!("py-4")}>{error_message}</p>
                        <div class={classes!("modal-action")}>
                            <form method="dialog">
                                <button class={classes!("btn")}>{"Close"}</button>
                            </form>
                        </div>
                    </div>
                </dialog>
            </>
        }
    }
}
