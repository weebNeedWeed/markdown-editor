use yew::prelude::*;

const DEFAULT_SVG_CLASS: &'static str = "h-4 w-4 text-white";

const VIEWBOX: AttrValue = AttrValue::Static("0 0 16 16");
const WIDTH: AttrValue = AttrValue::Static("16");
const HEIGHT: AttrValue = AttrValue::Static("16");

#[derive(Properties, PartialEq, Clone)]
struct SvgProps {
    #[prop_or(classes!(DEFAULT_SVG_CLASS))]
    class: Classes,
    children: Html,
}

#[function_component(Svg)]
fn svg(SvgProps { class, children }: &SvgProps) -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg"
            class={class.clone()}
            viewBox={VIEWBOX}
            width={WIDTH}
            height={HEIGHT}>
            {children.clone()}
        </svg>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct IconProps {
    #[prop_or(classes!(DEFAULT_SVG_CLASS))]
    pub class: Classes,
}

#[function_component(PlusSquare)]
pub fn plus_square(IconProps { class }: &IconProps) -> Html {
    html! {
        <Svg class={class.clone()}>
            <path d="M14 1a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1zM2 0a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2z"/>
            <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4"/>
        </Svg>
    }
}

#[function_component(CaretDownFill)]
pub fn caret_down_fill(IconProps { class }: &IconProps) -> Html {
    html! {
        <Svg class={class.clone()}>
            <path d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z"/>
        </Svg>
    }
}
