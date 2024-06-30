mod app;
mod header;
mod chr;
mod rom;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
