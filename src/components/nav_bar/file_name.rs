use crate::{
    contexts::markdown_context::{use_markdown, Markdown},
    utils::icons::FileText,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(FileName)]
pub fn file_name() -> Html {
    let markdown = use_markdown();
    let node_ref = use_node_ref();
    let handle_change_file_name = {
        let markdown = markdown.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            let mut value = input.value();
            if value.is_empty() {
                value.push_str("no_name.md");
            }
            let node_ref = node_ref.cast::<HtmlInputElement>().unwrap();
            node_ref.set_value(&value);
            let new_key = Some(AttrValue::from(value));
            let new_md = Markdown::new(new_key, markdown.text.clone());
            markdown.remove_from_storage().unwrap();
            new_md.save_to_storage().unwrap();
            markdown.set_markdown(new_md);
        })
    };

    {
        let node_ref = node_ref.clone();
        let markdown = markdown.clone();
        use_effect(move || {
            let node_ref = node_ref.cast::<HtmlInputElement>().unwrap();
            node_ref.set_value(&markdown.key.clone().unwrap());
        });
    }
    html! {
        <div class="flex items-center h-full">
            <span class="h-full flex items-center bg-skin-buttons px-2.5 rounded-s">
                <FileText class="fill-skin-typography" />
            </span>
            <input
                ref={node_ref}
                onchange={handle_change_file_name}
                type="text"
                class="h-full font-semibold w-auto border-[0.2rem] px-2.5 border-l-0 border-skin-buttons rounded-e outline-none text-black" />
        </div>
    }
}
