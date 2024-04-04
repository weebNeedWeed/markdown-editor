use crate::components::button::*;
use crate::contexts::markdown_context::use_markdown;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

#[function_component(SaveDropdown)]
pub fn save_dropdown() -> Html {
    let md_state = use_markdown().state();
    let encoded_md = urlencoding::encode(md_state.text.as_str());
    let text_dl = format!("data:text/markdown,{}", encoded_md);
    let download_name = md_state.key.unwrap();

    let handle_download = Callback::from(|_| {
        let anchor = gloo::utils::document()
            .get_element_by_id("dl")
            .unwrap()
            .unchecked_into::<HtmlAnchorElement>();
        anchor.click();
    });

    html! {
        <>
            <Button onclick={handle_download}>
                <span class="text-md text-skin-typography font-medium">
                    {"Lưu"}
                </span>
            </Button>

            <a id="dl" class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </>
    }
}
