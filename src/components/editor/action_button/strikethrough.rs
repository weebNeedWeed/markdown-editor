use crate::components::editor::action_button::ActionButton;
use crate::contexts::markdown_context::{use_markdown, Markdown};
use crate::utils::icons::TypeStrikethrough;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Strikethrough)]
pub fn strikethrough() -> Html {
    let markdown = use_markdown();
    let handle_click = Callback::from(move |_| {
        let textarea: HtmlTextAreaElement = gloo::utils::document()
            .get_element_by_id("edit_section")
            .unwrap()
            .dyn_into()
            .unwrap();
        let mut current_value = textarea.value();

        if let Some(end) = textarea.selection_end().unwrap() {
            let start = match textarea.selection_start().unwrap() {
                Some(x) => x,
                None => return,
            };
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "~~");
            current_value.insert_str(end_usize + 2, "~~");
            textarea.set_selection_end(Some(start + 2)).unwrap();
        } else {
            current_value.push_str("~~~~");
            textarea.set_selection_end(Some(4)).unwrap();
        }

        let md = Markdown::new(markdown.key.clone(), AttrValue::from(current_value));
        markdown.update_markdown(md).unwrap();
    });
    html! {
        <ActionButton
            onclick={handle_click}
            icon={
                html!{<TypeStrikethrough class="fill-skin-buttons w-5 h-5"/>}}
            title={"Strikethrough"}
        />
    }
}
