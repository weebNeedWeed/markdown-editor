use crate::{contexts::markdown_context::use_markdown, utils::icons::FiletypeMd};
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

#[function_component(SaveMarkdown)]
pub fn save_markdown() -> Html {
    let handle_download = Callback::from(|_| {
        let anchor = gloo::utils::document()
            .get_element_by_id("dl_md")
            .unwrap()
            .unchecked_into::<HtmlAnchorElement>();
        anchor.click();
    });

    let md_state = use_markdown().state();
    let encoded_md = urlencoding::encode(md_state.text.as_str());
    let text_dl = format!("data:text/markdown,{}", encoded_md);
    let download_name = md_state.key.unwrap();

    html! {
        <>
            <button onclick={handle_download} class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                <FiletypeMd class="stroke-skin-typography"/>
                {".md"}
            </button>

            <a id="dl_md" class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </>
    }
}
