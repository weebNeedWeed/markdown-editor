use crate::components::editor::action_button::ActionButton;
use crate::utils::icons::Arrow90DegLeft;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;

#[function_component(Undo)]
pub fn undo() -> Html {
    let handle_click = Callback::from(|_| {
        let htmldoc: HtmlDocument = gloo::utils::document().dyn_into().unwrap();
        htmldoc.exec_command("undo").unwrap();
    });
    html! {
        <ActionButton
            icon={
                html!{<Arrow90DegLeft class="fill-skin-buttons w-5 h-5"/>}}
            title={"Hoàn tác"}
            onclick={handle_click}
        />
    }
}
