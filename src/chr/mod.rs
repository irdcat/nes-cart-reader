pub mod data;

use std::cmp;

use yew::prelude::*;

use super::ui::{canvas::Canvas, input::ColorInput, pagination::Pagination, r#box::Box};
use data::{ChrData, PatternTable};

fn render_pattern_table(pattern_table: &PatternTable, colors: &Vec<u32>) {
    let canvas = Canvas::get_by_id("canvas".to_owned());
    let image_data = pattern_table.to_image_data((*colors).clone());
    Canvas::render_image_data(&canvas, image_data);
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
    let pattern_table_count = props
        .chr_data
        .as_ref()
        .map(|data| data.pattern_tables.len())
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

    let chr_data = props.chr_data.clone();
    let change_callback = {
        let chr_data = chr_data.clone();
        let colors = (*colors).clone();
        if chr_data.is_some() {
            let pattern_tables = chr_data.unwrap().pattern_tables.clone();
            Callback::from(move |page: usize| {
                log::info!("on_change {}", page);
                let index = cmp::min(page, pattern_table_count);
                let pattern_table = pattern_tables[index];
                render_pattern_table(&pattern_table, &colors);
            })
        } else {
            Callback::from(|_: usize| {})
        }
    };

    use_effect({
        let chr_data = props.chr_data.clone();
        let colors = colors.clone();
        move || {
            log::info!("Chr use_effect");
            chr_data
                .map(|d| d.pattern_tables)
                .map(|t| t[0])
                .inspect(|pt| {
                    render_pattern_table(pt, &colors);
                });
            || ()
        }
    });

    html! {
        <Box class={classes!("flex", "box-border", "border", "border-base-300")}>
            <Box class={classes!("grow")}>
                <Pagination count={pattern_table_count} on_change={change_callback}/>
                <Box>
                    <Canvas id="canvas" width={256} height={256} class={classes!("bg-black")}/>
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
