use yew::prelude::*;

use crate::rom::header::RomHeader;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub rom_header: Option<RomHeader>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <fieldset>
            <legend>{"Header Data"}</legend>
            <p>
                {"PRG ROM Size: "}
                {props.rom_header.as_ref().map(|v| v.prg_rom_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"CHR ROM Size: "}
                {props.rom_header.as_ref().map(|v| v.chr_rom_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"Mapper No: "}
                {props.rom_header.as_ref().map(|v| v.mapper.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"Submapper No: "}
                {props.rom_header.as_ref().map(|v| v.submapper.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"Trainer present: "}
                {props.rom_header.as_ref().map(|v| (if v.trainer_present { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"Mirroring Type: "}
                {props.rom_header.as_ref().map(|v| v.mirroring.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"iNES Header format: "}
                {props.rom_header.as_ref().map(|v| (if v.nes2 { "NES 2.0" } else { "NES 1.0" }).to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"PRG RAM Size: "}
                {props.rom_header.as_ref().map(|v| v.prg_ram_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"CHR RAM Size: "}
                {props.rom_header.as_ref().map(|v| v.chr_ram_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"PRG NVRAM Size: "}
                {props.rom_header.as_ref().map(|v| v.prg_nvram_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"CHR NVRAM Size: "}
                {props.rom_header.as_ref().map(|v| v.chr_nvram_size.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"TV System: "}
                {props.rom_header.as_ref().map(|v| v.tv_system.to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"PRG RAM Present: "}
                {props.rom_header.as_ref().map(|v| (if v.prg_ram_present { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}
            </p>
            <p>
                {"Has Bus Conflicts: "}
                {props.rom_header.as_ref().map(|v| (if v.has_bus_conflicts { "Yes" } else { "No" }).to_string()).unwrap_or("Not available".to_string())}
            </p>
        </fieldset>
    }
}
