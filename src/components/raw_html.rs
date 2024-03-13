use web_sys::Node;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct RawHtmlProps {
    pub inner_html: String,
}

#[function_component(RawHtml)]
pub fn raw_html(RawHtmlProps { inner_html }: &RawHtmlProps) -> Html {
    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_inner_html(inner_html);
    Html::VRef(Node::from(div))
}
