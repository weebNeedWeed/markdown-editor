use crate::components::editor::action_button::ActionButton;
use crate::utils::icons::Arrow90DegRight;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;

#[function_component(Redo)]
pub fn redo() -> Html {
    let handle_click = Callback::from(|_| {
        let htmldoc: HtmlDocument = gloo::utils::document().dyn_into().unwrap();
        htmldoc.exec_command("redo").unwrap();
    });
    html! {
        <ActionButton
            icon={
                html!{<Arrow90DegRight class="fill-skin-buttons w-5 h-5"/>}}
            title={"Redo"}
            onclick={handle_click}
        />
    }
}
