use crate::components::editor::action_button::ActionButton;
use crate::components::modal::Modal;
use crate::contexts::markdown_context::{use_markdown, Markdown};
use crate::utils::icons::ImageIcon;
use js_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component(Image)]
pub fn image() -> Html {
    let display_modal = use_state(|| false);
    let handle_toggle_modal = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(!*display_modal))
    };

    let markdown = use_markdown();
    let title = use_state(|| String::new());
    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            title.set(input.value())
        })
    };

    let value = use_state(|| String::new());
    let on_value_change = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            value.set(input.value())
        })
    };

    let handle_submit = {
        let title = title.clone();
        let value = value.clone();
        let display_modal = display_modal.clone();
        Callback::from(move |_| {
            let textarea: HtmlTextAreaElement = gloo::utils::document()
                .get_element_by_id("edit_section")
                .unwrap()
                .dyn_into()
                .unwrap();
            let mut current_value = textarea.value();

            let ins_value = format!("![{}]({})", *title, *value);

            if let Some(start) = textarea.selection_start().unwrap() {
                let start_usize = start as usize;

                current_value.insert_str(start_usize, &ins_value);
                textarea
                    .set_selection_end(Some(start + ins_value.len() as u32))
                    .unwrap();
            } else {
                current_value.push_str(&ins_value);
                textarea
                    .set_selection_end(Some(ins_value.len() as u32))
                    .unwrap();
            }

            let md = Markdown::new(markdown.key.clone(), AttrValue::from(current_value));
            markdown.update_markdown(md).unwrap();
            display_modal.set(false);
        })
    };

    html! {
        <>
            <ActionButton
                onclick={handle_toggle_modal.clone()}
                icon={
                    html!{<ImageIcon class="fill-skin-buttons w-5 h-5"/>}}
                title={"Ảnh"}
            />
            <Modal onsubmit={handle_submit} submit_title={"Chèn"} title={"Chèn hình ảnh"} open={*display_modal} onclose={handle_toggle_modal}>
                <div class="flex flex-col h-full overflow-auto py-2 gap-y-2 text-skin-primary">
                    <span class="text-sm">
                        {"Chèn hình ảnh vào Markdown của bạn"}
                    </span>

                    <label for="link_title" class="mt-2 text-lg">
                        {"Tiêu đề hình ảnh"}
                    </label>
                    <input
                        oninput={on_title_change}
                        type="text"
                        value={(*title).clone()}
                        id="link_title"
                        class="rounded w-3/4 border-2 border-skin-primary outline-none px-2 py-2.5"/>

                    <label for="link_value" class="mt-2 text-lg">
                        {"Liên kết hình ảnh"}
                    </label>
                    <input
                        value={(*value).clone()}
                        oninput={on_value_change}
                        type="text"
                        id="link_value"
                        class="rounded w-3/4 border-2 border-skin-primary outline-none px-2 py-2.5"/>

                </div>
            </Modal>
        </>
    }
}
