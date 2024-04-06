use crate::contexts::markdown_context::{use_markdown, Markdown};
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Heading)]
pub fn heading() -> Html {
    html! {
        <div class="group relative cursor-pointer">
            <span class="font-semibold text-skin-primary px-2.5 text-lg leading-3 py-1"> {"H"} </span>
            <div class="absolute top-full left-[-1rem] w-32 p-2 rounded bg-skin-primary hidden group-hover:block text-skin-typography shadow-md">
                <div class="w-full flex flex-col gap-y-2">
                    <HeadingButton title={AttrValue::from("Tiêu đề 1")} insert_value={String::from("# ")} />
                    <HeadingButton title={AttrValue::from("Tiêu đề 2")} insert_value={String::from("## ")} />
                    <HeadingButton title={AttrValue::from("Tiêu đề 3")} insert_value={String::from("### ")} />
                    <HeadingButton title={AttrValue::from("Tiêu đề 4")} insert_value={String::from("#### ")} />
                </div>
            </div>
        </div>
    }
}

#[yew_autoprops::autoprops]
#[function_component(HeadingButton)]
pub fn heading_button(title: AttrValue, insert_value: &String) -> Html {
    let markdown = use_markdown();
    let make_onclick_cb = move |txt: String| {
        Callback::from(move |_: MouseEvent| {
            let txt = txt.clone();
            let num = txt.len() as u32;
            let text_area: HtmlTextAreaElement = gloo::utils::document()
                .get_element_by_id("edit_section")
                .unwrap()
                .dyn_into()
                .unwrap();

            let mut current_value = text_area.value();

            if let Some(end) = text_area.selection_end().unwrap() {
                let start = match text_area.selection_start().unwrap() {
                    Some(x) => x,
                    None => return,
                };
                let start_usize = start as usize;
                let _end_usize = end as usize;

                current_value.insert_str(start_usize, &txt);
                text_area.set_selection_end(Some(start + num)).unwrap();
            } else {
                current_value.push_str(&txt);
                text_area.set_selection_end(Some(num)).unwrap();
            }

            let md = Markdown::new(markdown.key.clone(), AttrValue::from(current_value));
            markdown.update_markdown(md).unwrap();
        })
    };
    html! {
        <button onclick={make_onclick_cb.clone()(String::from(insert_value))} class="w-full h-6 flex items-center text-sm justify-around border-b-2 border-skin-primary hover:border-skin-typography transition-all">
            <span class="text-skin-typography">{"H"}{insert_value.len()}</span>
            {title}
        </button>
    }
}
