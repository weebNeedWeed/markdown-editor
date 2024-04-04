use crate::components::editor::action_button::ActionButton;
use crate::contexts::markdown_context::{use_markdown, Markdown};
use crate::utils::icons::TableIcon;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Table)]
pub fn table() -> Html {
    let markdown = use_markdown();
    let handle_click = Callback::from(move |_| {
        let textarea: HtmlTextAreaElement = gloo::utils::document()
            .get_element_by_id("edit_section")
            .unwrap()
            .dyn_into()
            .unwrap();
        let mut current_value = textarea.value();

        let ins_value = "|   |   |\n|--|--|\n|   |   |";

        if let Some(start) = textarea.selection_start().unwrap() {
            let start_usize = start as usize;

            current_value.insert_str(start_usize, ins_value);
            textarea
                .set_selection_end(Some(start + ins_value.len() as u32))
                .unwrap();
        } else {
            current_value.push_str(ins_value);
            textarea
                .set_selection_end(Some(ins_value.len() as u32))
                .unwrap();
        }

        let md = Markdown::new(markdown.key.clone(), AttrValue::from(current_value));
        markdown.update_markdown(md).unwrap();
    });
    html! {
        <ActionButton
            onclick={handle_click}
            icon={
                html!{<TableIcon class="fill-skin-buttons w-5 h-5"/>}}
            title={"Table"}
        />
    }
}
