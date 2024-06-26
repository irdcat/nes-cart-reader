mod rom;
mod app;
mod header;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}