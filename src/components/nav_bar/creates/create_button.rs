use crate::{
    contexts::markdown_context::{use_markdown, Markdown},
    utils::icons::PlusSquare,
};
use yew::prelude::*;

#[function_component(CreateButton)]
pub fn create_button() -> Html {
    let markdown = use_markdown();
    let handle_click = Callback::from(move |_| {
        let md = Markdown::default();
        markdown.set_markdown(md);
    });
    html! {
        <button onclick={handle_click} class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
            <PlusSquare class="stroke-skin-typography"/>
            {"Tạo mới"}
        </button>
    }
}
