use crate::{
    contexts::markdown_context::{use_markdown, Markdown},
    utils::icons::CloudArrowUp,
};
use gloo::file::Blob;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(UploadButton)]
pub fn upload_button() -> Html {
    let markdown = use_markdown();
    let handle_change = Callback::from(move |e: Event| {
        let input = e.target_unchecked_into::<HtmlInputElement>();
        let files = input.files().unwrap();
        let first_file = files.get(0).unwrap();
        let key = Some(AttrValue::from(first_file.name()));
        let blob = Blob::from(first_file);
        input.set_value("");
        let markdown = markdown.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let text = gloo::file::futures::read_as_text(&blob).await.unwrap();
            let md = Markdown::new(key, AttrValue::from(text));
            markdown.update_markdown(md).unwrap();
            markdown.save_to_storage().unwrap();
        });
    });

    html! {
        <form action="w-full">
            <label for="upload_button" class="cursor-pointer gap-x-2 w-full py-2 transition-all justify-center hover:opacity-80 flex items-center">
                <CloudArrowUp class="stroke-skin-typography"/>
                {"Tải lên"}
            </label>
            <input type="file"
                onchange={handle_change}
                id="upload_button"
                class="hidden"
                accept="text/*" />
        </form>
    }
}
