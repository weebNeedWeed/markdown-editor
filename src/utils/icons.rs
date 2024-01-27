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

#[function_component(CloudArrowUp)]
pub fn cloud_arrow_up(IconProps { class }: &IconProps) -> Html {
    html! {
        <Svg class={class.clone()}>
            <path fill-rule="evenodd" d="M7.646 5.146a.5.5 0 0 1 .708 0l2 2a.5.5 0 0 1-.708.708L8.5 6.707V10.5a.5.5 0 0 1-1 0V6.707L6.354 7.854a.5.5 0 1 1-.708-.708z"/>
            <path d="M4.406 3.342A5.53 5.53 0 0 1 8 2c2.69 0 4.923 2 5.166 4.579C14.758 6.804 16 8.137 16 9.773 16 11.569 14.502 13 12.687 13H3.781C1.708 13 0 11.366 0 9.318c0-1.763 1.266-3.223 2.942-3.593.143-.863.698-1.723 1.464-2.383m.653.757c-.757.653-1.153 1.44-1.153 2.056v.448l-.445.049C2.064 6.805 1 7.952 1 9.318 1 10.785 2.23 12 3.781 12h8.906C13.98 12 15 10.988 15 9.773c0-1.216-1.02-2.228-2.313-2.228h-.5v-.5C12.188 4.825 10.328 3 8 3a4.53 4.53 0 0 0-2.941 1.1z"/>
        </Svg>
    }
}

#[function_component(FiletypeMd)]
pub fn filetype_md(IconProps { class }: &IconProps) -> Html {
    html! {
        <Svg class={class.clone()}>
            <path fill-rule="evenodd" d="M14 4.5V14a2 2 0 0 1-2 2H9v-1h3a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5zM.706 13.189v2.66H0V11.85h.806l1.14 2.596h.026l1.14-2.596h.8v3.999h-.716v-2.66h-.038l-.946 2.159h-.516l-.952-2.16H.706Zm3.919 2.66V11.85h1.459q.609 0 1.005.234t.589.68q.195.445.196 1.075 0 .634-.196 1.084-.197.451-.595.689-.396.237-1 .237H4.626Zm1.353-3.354h-.562v2.707h.562q.279 0 .484-.082a.8.8 0 0 0 .334-.252 1.1 1.1 0 0 0 .196-.422q.067-.252.067-.592a2.1 2.1 0 0 0-.117-.753.9.9 0 0 0-.354-.454q-.238-.152-.61-.152"/>
        </Svg>
    }
}

#[function_component(FiletypePdf)]
pub fn filetype_pdf(IconProps { class }: &IconProps) -> Html {
    html! {
        <Svg class={class.clone()}>
            <path fill-rule="evenodd" d="M14 4.5V14a2 2 0 0 1-2 2h-1v-1h1a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5zM1.6 11.85H0v3.999h.791v-1.342h.803q.43 0 .732-.173.305-.175.463-.474a1.4 1.4 0 0 0 .161-.677q0-.375-.158-.677a1.2 1.2 0 0 0-.46-.477q-.3-.18-.732-.179m.545 1.333a.8.8 0 0 1-.085.38.57.57 0 0 1-.238.241.8.8 0 0 1-.375.082H.788V12.48h.66q.327 0 .512.181.185.183.185.522m1.217-1.333v3.999h1.46q.602 0 .998-.237a1.45 1.45 0 0 0 .595-.689q.196-.45.196-1.084 0-.63-.196-1.075a1.43 1.43 0 0 0-.589-.68q-.396-.234-1.005-.234zm.791.645h.563q.371 0 .609.152a.9.9 0 0 1 .354.454q.118.302.118.753a2.3 2.3 0 0 1-.068.592 1.1 1.1 0 0 1-.196.422.8.8 0 0 1-.334.252 1.3 1.3 0 0 1-.483.082h-.563zm3.743 1.763v1.591h-.79V11.85h2.548v.653H7.896v1.117h1.606v.638z"/>
        </Svg>
    }
}
