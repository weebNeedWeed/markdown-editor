use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::contexts::markdown_context::{use_markdown, Markdown};

#[function_component(EditSection)]
pub fn edit_section() -> Html {
    let markdown = use_markdown();
    let on_input = {
        let markdown = markdown.clone();
        Callback::from(move |e: InputEvent| {
            let textarea = e.target_unchecked_into::<HtmlTextAreaElement>();
            let md = Markdown::new(markdown.key.clone(), AttrValue::from(textarea.value()));
            markdown.update_markdown(md).unwrap();
        })
    };

    // Handle some special keys like Tab
    let on_key_down = {
        let markdown = markdown.clone();
        Callback::from(move |e: KeyboardEvent| {
            let target = e.target_dyn_into::<HtmlTextAreaElement>();
            if let Option::None = target {
                return;
            }

            let target = target.unwrap();
            let mut current_value = target.value();
            if e.key().eq("Tab") {
                e.prevent_default();
                if let Some(end) = target.selection_end().unwrap() {
                    let end_usize = end as usize;
                    current_value.insert_str(end_usize, "    ");
                    target.set_value(&current_value);
                    target.set_selection_end(Some(end + 4)).unwrap();
                } else {
                    current_value.push_str("    ");
                    target.set_value(&current_value);
                    target.set_selection_end(Some(4)).unwrap();
                }
                let md = Markdown::new(markdown.key.clone(), AttrValue::from(current_value));
                markdown.update_markdown(md).unwrap();
            }
        })
    };
    html! {
        <div class="w-full h-full overflow-y-hidden pb-4 pt-2">
            <textarea
                value={markdown.state().text}
                oninput={on_input}
                onkeydown={on_key_down}
                class="w-full min-h-full px-5 resize-none outline-none font-normal text-lg">
            </textarea>
        </div>
    }
}
