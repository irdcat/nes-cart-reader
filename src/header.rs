use yew::prelude::*;

use crate::rom::header::RomHeader;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub rom_header: Option<RomHeader>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <table class={classes!("table", "table-sm")}>
            <thead>
                <tr>
                    <th colspan="2">{"Header Data"}</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{"PRG ROM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.prg_rom_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"CHR ROM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.chr_rom_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"Mapper No: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.mapper.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"Submapper No: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.submapper.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"Trainer present: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| (if v.trainer_present { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"Mirroring Type: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.mirroring.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"iNES Header format: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| (if v.nes2 { "NES 2.0" } else { "NES 1.0" }).to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"PRG RAM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.prg_ram_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"CHR RAM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.chr_ram_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"PRG NVRAM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.prg_nvram_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"CHR NVRAM Size: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.chr_nvram_size.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"TV System: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| v.tv_system.to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"PRG RAM Present: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| (if v.prg_ram_present { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
                <tr>
                    <td>{"Has Bus Conflicts: "}</td>
                    <td>{props.rom_header.as_ref().map(|v| (if v.has_bus_conflicts { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}</td>
                </tr>
            </tbody>
        </table>
    }
}
