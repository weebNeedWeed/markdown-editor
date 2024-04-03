use crate::contexts::markdown_context::{use_markdown, Markdown};
use crate::utils::icons::*;
use js_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(Heading)]
pub fn heading() -> Html {
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
        <div class="group relative cursor-pointer">
            <span class="font-semibold text-lg text-skin-primary px-2 py-1"> {"H"} </span>
            <div class="absolute top-full left-[-1rem] w-28 p-2 rounded bg-skin-primary hidden group-hover:block text-skin-typography shadow-md">
                <div class="w-full flex flex-col gap-y-2">
                    <button onclick={make_onclick_cb.clone()(String::from("# "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH1 class="fill-skin-typography"/>
                        {"Heading 1"}
                    </button>

                    <button onclick={make_onclick_cb.clone()(String::from("## "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH2 class="fill-skin-typography"/>
                        {"Heading 2"}
                    </button>

                    <button onclick={make_onclick_cb.clone()(String::from("### "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH3 class="fill-skin-typography"/>
                        {"Heading 3"}
                    </button>

                    <button onclick={make_onclick_cb.clone()(String::from("#### "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH4 class="fill-skin-typography"/>
                        {"Heading 4"}
                    </button>

                    <button onclick={make_onclick_cb.clone()(String::from("##### "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH5 class="fill-skin-typography"/>
                        {"Heading 5"}
                    </button>

                    <button onclick={make_onclick_cb.clone()(String::from("###### "))} class="w-full h-5 flex items-center text-sm justify-around">
                        <TypeH6 class="fill-skin-typography"/>
                        {"Heading 6"}
                    </button>
                </div>
            </div>
        </div>
    }
}
