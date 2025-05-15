pub mod data;
pub mod instructions;

use std::collections::{BTreeMap, HashMap};

use yew::prelude::*;

use data::PrgData;
use instructions::{Index, Instruction, Mnemonic, Operand};

#[derive(Properties, PartialEq)]
pub struct PrgProps {
    pub prg_data: Option<PrgData>,
}

#[function_component(Prg)]
pub fn prg(_props: &PrgProps) -> Html {
    //let prg_data_clone = props.prg_data.clone();

    use Index::*;
    use Mnemonic::*;
    use Operand::*;
    let prg_data_clone = Some(PrgData {
        instructions: HashMap::from([
            (
                0x0000,
                Instruction {
                    mnemonic: Ora,
                    operand: Some(ZeroPage { address: 0x0B }),
                    bytes: vec![0x05, 0x0B],
                },
            ),
            (
                0x0002,
                Instruction {
                    mnemonic: Jmp,
                    operand: Some(Indirect { address: 0x0201 }),
                    bytes: vec![0x6C, 0x01, 0x02],
                },
            ),
            (
                0x0005,
                Instruction {
                    mnemonic: Asl,
                    operand: None,
                    bytes: vec![0x0A],
                },
            ),
            (
                0x0006,
                Instruction {
                    mnemonic: Ldx,
                    operand: Some(Immediate { value: 0xFF }),
                    bytes: vec![0xA2, 0xFF],
                },
            ),
            (
                0x0008,
                Instruction {
                    mnemonic: Jsr,
                    operand: Some(Absolute { address: 0xFD02 }),
                    bytes: vec![0x20, 0x02, 0xFD],
                },
            ),
            (
                0x000B,
                Instruction {
                    mnemonic: Sei,
                    operand: None,
                    bytes: vec![0x78],
                },
            ),
            (
                0x000C,
                Instruction {
                    mnemonic: Bne,
                    operand: Some(Absolute { address: 0x000A }),
                    bytes: vec![0xD0, 0xFC],
                },
            ),
            (
                0x000E,
                Instruction {
                    mnemonic: Ora,
                    operand: Some(AbsoluteIndexed {
                        address: 0x1E05,
                        index: X,
                    }),
                    bytes: vec![0x1D, 0x05, 0x1E],
                },
            ),
            (
                0x0011,
                Instruction {
                    mnemonic: Nop,
                    operand: Some(ZeroPage { address: 0x15 }),
                    bytes: vec![0x04, 0x15],
                },
            ),
            (
                0x0013,
                Instruction {
                    mnemonic: Stp,
                    operand: None,
                    bytes: vec![0x02],
                },
            ),
            (
                0x0014,
                Instruction {
                    mnemonic: Stx,
                    operand: Some(ZeroPageIndexed {
                        address: 0xAB,
                        index: Y,
                    }),
                    bytes: vec![0x96, 0xAB],
                },
            ),
            (
                0x0016,
                Instruction {
                    mnemonic: Cli,
                    operand: None,
                    bytes: vec![0x58],
                },
            ),
            (
                0x0017,
                Instruction {
                    mnemonic: Adc,
                    operand: Some(PreIndexedIndirect { address: 0x01 }),
                    bytes: vec![0x61, 0x01],
                },
            ),
            (
                0x0019,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x001C,
                Instruction {
                    mnemonic: Ora,
                    operand: Some(ZeroPage { address: 0x0B }),
                    bytes: vec![0x05, 0x0B],
                },
            ),
            (
                0x001E,
                Instruction {
                    mnemonic: Jmp,
                    operand: Some(Indirect { address: 0x0201 }),
                    bytes: vec![0x6C, 0x01, 0x02],
                },
            ),
            (
                0x0022,
                Instruction {
                    mnemonic: Asl,
                    operand: None,
                    bytes: vec![0x0A],
                },
            ),
            (
                0x0023,
                Instruction {
                    mnemonic: Ldx,
                    operand: Some(Immediate { value: 0xFF }),
                    bytes: vec![0xA2, 0xFF],
                },
            ),
            (
                0x0025,
                Instruction {
                    mnemonic: Jsr,
                    operand: Some(Absolute { address: 0xFD02 }),
                    bytes: vec![0x20, 0x02, 0xFD],
                },
            ),
            (
                0x0028,
                Instruction {
                    mnemonic: Sei,
                    operand: None,
                    bytes: vec![0x78],
                },
            ),
            (
                0x0029,
                Instruction {
                    mnemonic: Bne,
                    operand: Some(Absolute { address: 0x000A }),
                    bytes: vec![0xD0, 0xFC],
                },
            ),
            (
                0x002A,
                Instruction {
                    mnemonic: Ora,
                    operand: Some(AbsoluteIndexed {
                        address: 0x1E05,
                        index: X,
                    }),
                    bytes: vec![0x1D, 0x05, 0x1E],
                },
            ),
            (
                0x002D,
                Instruction {
                    mnemonic: Nop,
                    operand: Some(ZeroPage { address: 0x15 }),
                    bytes: vec![0x04, 0x15],
                },
            ),
            (
                0x0030,
                Instruction {
                    mnemonic: Stp,
                    operand: None,
                    bytes: vec![0x02],
                },
            ),
            (
                0x0031,
                Instruction {
                    mnemonic: Stx,
                    operand: Some(ZeroPageIndexed {
                        address: 0xAB,
                        index: Y,
                    }),
                    bytes: vec![0x96, 0xAB],
                },
            ),
            (
                0x0033,
                Instruction {
                    mnemonic: Cli,
                    operand: None,
                    bytes: vec![0x58],
                },
            ),
            (
                0x0034,
                Instruction {
                    mnemonic: Adc,
                    operand: Some(PreIndexedIndirect { address: 0x01 }),
                    bytes: vec![0x61, 0x01],
                },
            ),
            (
                0x0036,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x0038,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x003A,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x003C,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x003E,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x0040,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x0042,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x0044,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
            (
                0x0046,
                Instruction {
                    mnemonic: Sta,
                    operand: Some(PostIndexedIndirect { address: 0xFB }),
                    bytes: vec![0x91, 0xFB],
                },
            ),
        ]),
    });

    html! {
        <div class={classes!("h-full", "box-border", "border", "border-base-300", )}>
            <div class={classes!("table", "w-1/2")}>
            {
                prg_data_clone
                    .map_or(HashMap::new(), |prg_data| prg_data.instructions)
                    .into_iter()
                    .collect::<BTreeMap<u16, Instruction>>()
                    .into_iter()
                    .map(|(address, instruction)| {
                        html! {
                            <div class={classes!("table-row")}>
                                <div class={classes!("table-cell")}>{format!("${:04X}:", address)}</div>
                                <div class={classes!("table-cell")}>
                                {
                                    instruction.bytes
                                        .into_iter()
                                        .map(|v| format!("{:02X}", v))
                                        .collect::<Vec<String>>()
                                        .join(" ")
                                }
                                </div>
                                <div class={classes!("table-cell")}>{instruction.mnemonic.to_string()}</div>
                                <div class={classes!("table-cell")}>{instruction.operand.map_or(String::new(), |v| v.to_string())}</div>
                            </div>
                        }
                    })
                    .collect::<Html>()
            }
            </div>
        </div>
    }
}
