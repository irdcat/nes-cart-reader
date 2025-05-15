pub mod data;

use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, ImageBitmap, ImageData,
};
use yew::prelude::*;

use super::ui::{button::Button, r#box::Box};
use data::{ChrData, PatternTable, TILE_PATTERN_HEIGHT_IN_PIXELS, TILE_PATTERN_WIDTH_IN_PIXELS};

async fn render_pattern_table(pattern_table: PatternTable, colors: [u32; 4]) {
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

fn pattern_table_change_button_classes(enabled: bool) -> Classes {
    if enabled {
        classes!("join-item", "btn")
    } else {
        classes!("join-item", "btn", "btn-disabled")
    }
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

    let colors = use_state(|| [0xFF3030FFu32, 0x30FF30FFu32, 0x3030FFFFu32, 0xEFEFEFFFu32]);

    let chr_data_clone = props.chr_data.clone();
    let current_pattern_table_clone = current_pattern_table.clone();
    let colors_clone = colors.clone();
    use_effect(move || {
        if chr_data_clone.is_some() {
            let pattern_tables = chr_data_clone.unwrap().pattern_tables;
            if *current_pattern_table_clone >= pattern_tables.len() && !pattern_tables.is_empty() {
                current_pattern_table_clone.set(0);
            }
            let pattern_table = pattern_tables[*current_pattern_table_clone];
            spawn_local(render_pattern_table(pattern_table, *colors_clone))
        }
        || ()
    });

    html! {
        <Box class={classes!("flex", "box-border", "border", "border-base-300")}>
            <Box class={classes!("grow")}>
                <Box class={classes!("join", "flex", "justify-center")}>
                    <Button
                        class={pattern_table_change_button_classes(*current_pattern_table != 0)}
                        onclick={
                            let current_pattern_table = current_pattern_table.clone();
                            Callback::from(move |_: MouseEvent|{
                                if *current_pattern_table > 0 {
                                    current_pattern_table.set(*current_pattern_table - 1);
                                }
                            })
                        }>
                        {"«"}
                    </Button>
                    <Box class={
                        classes!(
                            "join-item", "h-12", "min-h-12",
                            "pl-4", "pr-4", "text-sm",
                            "font-semibold", "items-center", "inline-flex"
                        )}>
                        {format!("Page {}", *current_pattern_table)}
                    </Box>
                    <Button
                        class={pattern_table_change_button_classes(*current_pattern_table < last_pattern_table)}
                        onclick={
                            let current_pattern_table = current_pattern_table.clone();
                            Callback::from(move |_: MouseEvent|{
                                if *current_pattern_table < last_pattern_table {
                                    current_pattern_table.set(*current_pattern_table + 1);
                                }
                            })
                        }>
                        {"»"}
                    </Button>
                </Box>
                <Box>
                    <canvas id="canvas" width="256" height="256" class={classes!("bg-black")}></canvas>
                </Box>
            </Box>
            <Box class={classes!("grow-0", "flex", "flex-col", "p-3", "gap-3")}>
                <input
                    type="color"
                    class={classes!("paletteColorPicker", "grow-0")}
                    value={rgba_color_to_rgb_hex_string((*colors)[0])}
                    onchange={
                        let colors = colors.clone();
                        Callback::from(move |event: Event| {
                            colors.set({
                                let mut colors = *colors;
                                let value = event.target_dyn_into::<HtmlInputElement>().unwrap().value();
                                colors[0] = rgb_hex_string_to_rgba_color(value);
                                colors
                            });
                        })
                    }/>
                <input
                    type="color"
                    class={classes!("paletteColorPicker", "grow-0")}
                    value={rgba_color_to_rgb_hex_string((*colors)[1])}
                    onchange={
                        let colors = colors.clone();
                        Callback::from(move |event: Event| {
                            colors.set({
                                let mut colors = *colors;
                                let value = event.target_dyn_into::<HtmlInputElement>().unwrap().value();
                                colors[1] = rgb_hex_string_to_rgba_color(value);
                                colors
                            });
                        })
                    }/>
                <input
                    type="color"
                    class={classes!("paletteColorPicker", "grow-0")}
                    value={rgba_color_to_rgb_hex_string((*colors)[2])}
                    onchange={
                        let colors = colors.clone();
                        Callback::from(move |event: Event| {
                            colors.set({
                                let mut colors = *colors;
                                let value = event.target_dyn_into::<HtmlInputElement>().unwrap().value();
                                colors[2] = rgb_hex_string_to_rgba_color(value);
                                colors
                            });
                        })
                    }/>
                <input
                    type="color"
                    class={classes!("paletteColorPicker", "grow-0")}
                    value={rgba_color_to_rgb_hex_string((*colors)[3])}
                    onchange={
                        let colors = colors.clone();
                        Callback::from(move |event: Event| {
                            colors.set({
                                let mut colors = *colors;
                                let value = event.target_dyn_into::<HtmlInputElement>().unwrap().value();
                                colors[3] = rgb_hex_string_to_rgba_color(value);
                                colors
                            });
                        })
                    }/>
            </Box>
        </Box>
    }
}
