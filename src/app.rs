use std::collections::HashMap;

use gloo::file::{
    callbacks::{read_as_bytes, FileReader},
    File,
};
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::{html, Component, Context, Event, Html, TargetCast};

use crate::header::Header;
use crate::rom::{RomReader, RomReaderParams, RomReaderResult};

pub struct App {
    readers: HashMap<String, FileReader>,
    result: Option<RomReaderResult>,
}

pub enum AppMessage {
    FileUploaded(File),
    FileLoadSuccess(String, Vec<u8>),
    FileLoadFailure(String, String),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::with_capacity(1),
            result: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::FileUploaded(file) => {
                let link = ctx.link().clone();
                let uuid = Uuid::new_v4().to_string();
                let task = {
                    let uuid = uuid.clone();
                    read_as_bytes(&file, move |result| {
                        let msg = match result {
                            Ok(data) => AppMessage::FileLoadSuccess(uuid, data),
                            Err(err) => AppMessage::FileLoadFailure(uuid, err.to_string()),
                        };
                        link.send_message(msg);
                    })
                };
                self.readers.insert(uuid, task);
                true
            }
            AppMessage::FileLoadSuccess(uuid, bytes) => {
                let link = ctx.link().clone();
                match RomReader::read(RomReaderParams {
                    data: bytes,
                    origin: 0xC000,
                }) {
                    Ok(result) => {
                        self.result = Some(result);
                        self.readers.remove(&uuid);
                    }
                    Err(error) => {
                        link.send_message(AppMessage::FileLoadFailure(uuid, error.to_string()))
                    }
                }
                true
            }
            AppMessage::FileLoadFailure(uuid, _message) => {
                // TODO: Handle failure
                self.readers.remove(&uuid);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().unwrap().get(0).map(File::from).unwrap();
            AppMessage::FileUploaded(file)
        });

        html! {
            <>
                <nav>
                    <input
                        type="file"
                        multiple={false}
                        {onchange}
                    />
                </nav>
                <main>
                    <Header rom_header={ self.result.as_ref().map(|v| v.header) }/>
                </main>
            </>
        }
    }
}
