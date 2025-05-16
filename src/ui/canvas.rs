use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap, ImageData};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub class: Classes,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    html! {
        <canvas
            id={props.id.clone()}
            width={props.width.to_string()}
            height={props.height.to_string()}
            class={props.class.clone()}>
        </canvas>
    }
}

impl Canvas {
    pub fn get_by_id(id: String) -> HtmlCanvasElement {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id(id.as_str())
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
    }

    pub fn get_context_2d(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap()
    }

    pub fn scale_to(canvas: &HtmlCanvasElement, target_width: u32, target_height: u32) {
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        let scale_x = canvas_width as f64 / target_width as f64;
        let scale_y = canvas_height as f64 / target_height as f64;
        let context = Canvas::get_context_2d(canvas);
        context.scale(scale_x, scale_y).unwrap();
    }

    pub fn reset_transform(canvas: &HtmlCanvasElement) {
        let context = Canvas::get_context_2d(canvas);
        context.reset_transform().unwrap();
    }

    pub fn render_image_data(canvas: &HtmlCanvasElement, data: ImageData) {
        async fn map_and_render(canvas: HtmlCanvasElement, data: ImageData) {
            let bitmap = web_sys::window()
                .unwrap()
                .create_image_bitmap_with_image_data(&data)
                .map(JsFuture::from)
                .unwrap()
                .await
                .unwrap()
                .dyn_into::<ImageBitmap>()
                .unwrap();
            let context = Canvas::get_context_2d(&canvas);
            context
                .draw_image_with_image_bitmap(&bitmap, 0.0, 0.0)
                .unwrap();
        }
        spawn_local({
            let canvas = canvas.clone();
            map_and_render(canvas, data)
        });
    }
}
