use std::collections::HashMap;

use chr::Chr;
use gloo::file::{
    callbacks::{read_as_bytes, FileReader},
    File,
};
use header::Header;
use prg::Prg;
use reader::{RomReader, RomReaderResult};
use ui::{
    dialog::{AlertDialog, Dialog},
    input::FileInput,
    navbar::Navbar,
};
use uuid::Uuid;
use yew::prelude::*;

mod chr;
mod header;
mod prg;
mod reader;
mod ui;

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
                match RomReader::read(bytes) {
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
                Dialog::open_modal("romLoadDialog".into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(|f: File| AppMessage::Uploaded(f));

        let error_message = self.error.clone();
        let header_data_clone = self.result.as_ref().map(|v| v.header.clone());
        let chr_data_clone = self.result.as_ref().map(|v| v.chr_data.clone());
        let prg_data_clone = self.result.as_ref().map(|v| v.prg_data.clone());

        html! {
            <>
                <Navbar>
                    <FileInput id="rom-input" prompt="Load ROM" placeholder="Choose ROM" {on_change}/>
                </Navbar>
                <main class={classes!("flex")}>
                    <div class={classes!("grow-0")}>
                        <Header rom_header={ header_data_clone }/>
                        <Chr chr_data={ chr_data_clone }/>
                    </div>
                    <div class={classes!("grow")}>
                        <Prg prg_data={ prg_data_clone }/>
                    </div>
                </main>
                <AlertDialog id="romLoadDialog" title="Error" message={error_message}/>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
