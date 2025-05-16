pub mod data;

use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageBitmap, ImageData};
use yew::prelude::*;

use super::ui::{input::ColorInput, pagination::Pagination, r#box::Box};
use data::{ChrData, PatternTable, TILE_PATTERN_HEIGHT_IN_PIXELS, TILE_PATTERN_WIDTH_IN_PIXELS};

async fn render_pattern_table(pattern_table: PatternTable, colors: Vec<u32>) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let data = pattern_table.to_rgba_pixels(colors);
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&data),
        TILE_PATTERN_WIDTH_IN_PIXELS as u32,
        TILE_PATTERN_HEIGHT_IN_PIXELS as u32,
    )
    .unwrap();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let scale_x = canvas_width as f64 / TILE_PATTERN_WIDTH_IN_PIXELS as f64;
    let scale_y = canvas_height as f64 / TILE_PATTERN_HEIGHT_IN_PIXELS as f64;
    context.scale(scale_x, scale_y).unwrap();
    let image_bitmap_promise = web_sys::window()
        .unwrap()
        .create_image_bitmap_with_image_data(&image_data)
        .unwrap();
    let image_bitmap = JsFuture::from(image_bitmap_promise)
        .await
        .unwrap()
        .dyn_into::<ImageBitmap>()
        .unwrap();
    context
        .draw_image_with_image_bitmap(&image_bitmap, 0.0, 0.0)
        .unwrap();
    context.reset_transform().unwrap();
}

fn rgba_color_to_rgb_hex_string(color: u32) -> String {
    let rgb = color >> 8;
    format!(
        "#{:02X}{:02X}{:02X}",
        (rgb >> 16) & 0xFF,
        (rgb >> 8) & 0xFF,
        rgb & 0xFF
    )
}

fn rgb_hex_string_to_rgba_color(string: String) -> u32 {
    let rgb = u32::from_str_radix(&string[1..], 16).unwrap();
    (rgb << 8) | 0xFF
}

#[derive(Properties, PartialEq)]
pub struct ChrProps {
    pub chr_data: Option<ChrData>,
}

#[function_component(Chr)]
pub fn chr(props: &ChrProps) -> Html {
    let current_pattern_table = use_state(|| 0usize);
    let last_pattern_table = props
        .chr_data
        .as_ref()
        .map(|data| data.pattern_tables.len() - 1)
        .unwrap_or(0usize);

    let colors = use_state(|| vec![0xFF3030FFu32, 0x30FF30FFu32, 0x3030FFFFu32, 0xEFEFEFFFu32]);
    let color_callback = |idx: usize| {
        let colors = colors.clone();
        Callback::from(move |color: String| {
            colors.set({
                let mut colors = (*colors).clone();
                colors[idx] = rgb_hex_string_to_rgba_color(color);
                colors
            })
        })
    };

    let change_callback = {
        let current_pattern_table = current_pattern_table.clone();
        Callback::from(move |page: usize| {
            current_pattern_table.set(page);
        })
    };

    use_effect({
        let chr_data = props.chr_data.clone();
        let current_pattern_table = current_pattern_table.clone();
        let colors = colors.clone();
        move || {
            if chr_data.is_some() {
                let pattern_tables = chr_data.unwrap().pattern_tables;
                if *current_pattern_table >= pattern_tables.len() && !pattern_tables.is_empty() {
                    current_pattern_table.set(0);
                }
                let pattern_table = pattern_tables[*current_pattern_table];
                spawn_local(render_pattern_table(pattern_table, (*colors).clone()));
            }
            || ()
        }
    });

    html! {
        <Box class={classes!("flex", "box-border", "border", "border-base-300")}>
            <Box class={classes!("grow")}>
                <Pagination count={last_pattern_table} page={0} on_change={change_callback}/>
                <Box>
                    <canvas id="canvas" width="256" height="256" class={classes!("bg-black")}></canvas>
                </Box>
            </Box>
            <Box class={classes!("grow-0", "flex", "flex-col", "p-3", "gap-3")}>
                {
                    (*colors).clone()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, color)| {
                            html! {
                                <>
                                    <Box class={classes!("grow-0", "justify-center", "inline-flex")}>
                                        {idx}
                                    </Box>
                                    <ColorInput
                                        class={classes!("grow-0")}
                                        value={rgba_color_to_rgb_hex_string(color)}
                                        on_change={color_callback(idx)}/>
                                </>
                            }
                        })
                        .collect::<Html>()
                }
            </Box>
        </Box>
    }
}
