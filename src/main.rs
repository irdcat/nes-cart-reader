mod app;
mod header;
mod rom;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
