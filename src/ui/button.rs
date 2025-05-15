use yew::prelude::*;

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ButtonColor {
    Default,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}

fn color_to_class(color: &ButtonColor) -> &'static str {
    match color {
        ButtonColor::Default => "",
        ButtonColor::Neutral => "btn-neutral",
        ButtonColor::Primary => "btn-primary",
        ButtonColor::Secondary => "btn-secondary",
        ButtonColor::Accent => "btn-accent",
        ButtonColor::Info => "btn-info",
        ButtonColor::Success => "btn-success",
        ButtonColor::Warning => "btn-warning",
        ButtonColor::Error => "btn-error",
    }
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ButtonStyle {
    Default,
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
}

fn style_to_class(style: &ButtonStyle) -> &'static str {
    match style {
        ButtonStyle::Default => "",
        ButtonStyle::Outline => "btn-outline",
        ButtonStyle::Dash => "btn-dash",
        ButtonStyle::Soft => "btn-soft",
        ButtonStyle::Ghost => "btn-ghost",
        ButtonStyle::Link => "btn-link",
    }
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ButtonSize {
    Default,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

fn size_to_class(size: &ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "",
        ButtonSize::ExtraSmall => "btn-xs",
        ButtonSize::Small => "btn-sm",
        ButtonSize::Medium => "btn-md",
        ButtonSize::Large => "btn-lg",
        ButtonSize::ExtraLarge => "btn-xl",
    }
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ButtonShape {
    Default,
    Wide,
    Block,
    Square,
    Circle,
}

fn shape_to_class(shape: &ButtonShape) -> &'static str {
    match shape {
        ButtonShape::Default => "",
        ButtonShape::Wide => "btn-wide",
        ButtonShape::Block => "btn-block",
        ButtonShape::Square => "btn-square",
        ButtonShape::Circle => "btn-circle",
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Html,

    #[prop_or(Callback::from(move |_e: MouseEvent| {}))]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub id: String,

    #[prop_or(classes!())]
    pub class: Classes,

    #[prop_or(ButtonColor::Default)]
    pub color: ButtonColor,

    #[prop_or(ButtonStyle::Default)]
    pub style: ButtonStyle,

    #[prop_or(ButtonSize::Default)]
    pub size: ButtonSize,

    #[prop_or(ButtonShape::Default)]
    pub shape: ButtonShape,
}

fn props_to_classes(props: &ButtonProps) -> Classes {
    classes!(
        "btn",
        color_to_class(&props.color),
        style_to_class(&props.style),
        size_to_class(&props.size),
        shape_to_class(&props.shape)
    )
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            id={props.id.clone()}
            class={classes!(props_to_classes(props), props.class.clone())}
            onclick={props.onclick.clone()}>
            { props.children.clone() }
        </button>
    }
}
