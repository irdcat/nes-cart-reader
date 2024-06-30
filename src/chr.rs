use yew::prelude::*;

use crate::rom::chr_data::ChrData;

#[derive(Properties, PartialEq)]
pub struct ChrProps {
    pub chr_data: Option<ChrData>
}

#[function_component(Chr)]
pub fn chr(props: &ChrProps) -> Html {
    
    html! {
        <fieldset>
            <legend>{"CHR ROM Data"}</legend>
        </fieldset>
    }
}