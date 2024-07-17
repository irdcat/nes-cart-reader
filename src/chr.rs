use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement, ImageBitmap, ImageData,
};
use yew::prelude::*;

use crate::rom::chr_data::{ChrData, PatternTable};

fn pattern_table_to_rgba_pixels(pattern_table: PatternTable, colors: [u32; 4]) -> Vec<u8> {
    const TILES_PER_ROW: usize = 16;
    const TILE_WIDTH_IN_PIXELS: usize = 8;
    const TILE_ROWS: usize = 16;
    const TILE_HEIGHT_IN_PIXELS: usize = 8;
    const BITS_PER_PIXEL: usize = 2;

    const WIDTH_IN_PIXELS: usize = TILES_PER_ROW * TILE_WIDTH_IN_PIXELS;
    const HEIGHT_IN_PIXELS: usize = TILE_ROWS * TILE_HEIGHT_IN_PIXELS;

    let mut buffer = vec![0u8; WIDTH_IN_PIXELS * HEIGHT_IN_PIXELS * 4];

    for (index, tile) in pattern_table.iter().enumerate() {
        let column_index = index % TILES_PER_ROW;
        let row_index = index / TILES_PER_ROW;

        let start_position_x = column_index * TILE_WIDTH_IN_PIXELS;
        let start_position_y = row_index * TILE_HEIGHT_IN_PIXELS;

        for tile_row in 0..TILE_HEIGHT_IN_PIXELS {
            for tile_column in 0..TILE_WIDTH_IN_PIXELS {
                let row = tile.pattern[tile_row];
                let pixel =
                    (row >> (((TILE_WIDTH_IN_PIXELS - 1) - tile_column) * BITS_PER_PIXEL)) & 3;
                let position_x = start_position_x + tile_column;
                let position_y = start_position_y + tile_row;
                let buffer_index = (position_y * WIDTH_IN_PIXELS + position_x) * 4;
                let color = colors[pixel as usize];
                buffer[buffer_index] = ((color >> 24) & 0xFF) as u8;
                buffer[buffer_index + 1] = ((color >> 16) & 0xFF) as u8;
                buffer[buffer_index + 2] = ((color >> 8) & 0xFF) as u8;
                buffer[buffer_index + 3] = (color & 0xFF) as u8;
            }
        }
    }

    buffer
}

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

    const WIDTH: u32 = 128;
    const HEIGHT: u32 = 128;
    let data = pattern_table_to_rgba_pixels(pattern_table, colors);
    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), WIDTH, HEIGHT).unwrap();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();
    let scale_x = canvas_width as f64 / WIDTH as f64;
    let scale_y = canvas_height as f64 / HEIGHT as f64;
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
        "#{:02x}{:02x}{:02x}",
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
        <div class={classes!("flex")}>
            <div class={classes!("grow")}>
                <div class={classes!("join", "flex", "justify-center")}>
                    <button
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
                    </button>
                    <p class={
                        classes!(
                            "join-item", "h-12", "min-h-12",
                            "pl-4", "pr-4", "text-sm",
                            "font-semibold", "items-center", "inline-flex"
                        )}>
                        {format!("Page {}", *current_pattern_table)}
                    </p>
                    <button
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
                    </button>
                </div>
                <div>
                    <canvas id="canvas" width="256" height="256" class={classes!("bg-black")}></canvas>
                </div>
            </div>
            <div class={classes!("grow-0", "flex", "flex-col", "p-3", "gap-3")}>
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
            </div>
        </div>
    }
}
