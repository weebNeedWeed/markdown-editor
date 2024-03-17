use crate::utils::icons::FiletypeMd;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

#[function_component(SaveMarkdown)]
pub fn save_markdown() -> Html {
    let handle_download = Callback::from(|_| {
        let anchor = gloo::utils::document()
            .get_element_by_id("dl")
            .unwrap()
            .unchecked_into::<HtmlAnchorElement>();
        anchor.click();
    });
    html! {
        <button onclick={handle_download} class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
            <FiletypeMd class="stroke-skin-typography"/>
            {".md"}
        </button>
    }
}
