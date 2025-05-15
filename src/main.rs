mod app;
mod chr;
mod header;
mod prg;
mod rom;
mod ui;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
