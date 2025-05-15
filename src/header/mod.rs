pub mod data;

use super::ui::r#box::Box;
use data::HeaderData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub rom_header: Option<HeaderData>,
}

fn header_data_to_list(header: &Option<HeaderData>) -> Vec<(&'static str, String)> {
    vec![
        (
            "PRG ROM Size",
            header
                .as_ref()
                .map(|v| v.prg_rom_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "CHR ROM Size",
            header
                .as_ref()
                .map(|v| v.chr_rom_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "Mapper No",
            header
                .as_ref()
                .map(|v| v.mapper.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "Submapper No",
            header
                .as_ref()
                .map(|v| v.submapper.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "Trainer present",
            header
                .as_ref()
                .map(|v| (if v.trainer_present { "Yes" } else { "No" }).to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "Mirroring Type",
            header
                .as_ref()
                .map(|v| v.mirroring.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "iNES Header format",
            header
                .as_ref()
                .map(|v| (if v.nes2 { "NES 2.0" } else { "NES 1.0" }).to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "PRG RAM Size",
            header
                .as_ref()
                .map(|v| v.prg_ram_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "CHR RAM Size",
            header
                .as_ref()
                .map(|v| v.chr_ram_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "PRG NVRAM Size",
            header
                .as_ref()
                .map(|v| v.prg_nvram_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "CHR NVRAM Size",
            header
                .as_ref()
                .map(|v| v.chr_nvram_size.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "TV System",
            header
                .as_ref()
                .map(|v| v.tv_system.to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "PRG RAM Present",
            header
                .as_ref()
                .map(|v| (if v.prg_ram_present { "Yes" } else { "No" }).to_string())
                .unwrap_or("Not available".to_string()),
        ),
        (
            "Has Bus Conflicts",
            header
                .as_ref()
                .map(|v| (if v.has_bus_conflicts { "Yes" } else { "No" }).to_string())
                .unwrap_or("Not available".to_string()),
        ),
    ]
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let header_list = header_data_to_list(&props.rom_header);
    html! {
        <Box class={classes!("border", "border-base-300", "box-border")}>
            <Box class={classes!("text-xs")}>
                <Box class={classes!("flex", "px-1", "py-2")}>
                    <Box class={classes!("grow", "font-extrabold")}>
                        { "Header Data" }
                    </Box>
                </Box>
                {
                    header_list
                        .into_iter()
                        .map(|view| {
                            html! {
                                <Box class={classes!("flex", "px-1", "py-2")}>
                                    <Box class={classes!("grow", "font-medium")}>
                                        { view.0 }
                                    </Box>
                                    <Box class={classes!("grow-0")}>
                                        { view.1 }
                                    </Box>
                                </Box>
                            }
                        })
                        .collect::<Html>()
                }
            </Box>
        </Box>
    }
}
