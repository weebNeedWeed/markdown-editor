use crate::{contexts::markdown_context::use_markdown, utils::convert_to_html::convert_to_html};
use yew::prelude::*;

#[function_component(PreviewWindow)]
pub fn preview_window() -> Html {
    let classes = classes!(
        "prose",
        "prose-lg",
        "prose-img:rounded-xl",
        "prose-pre:bg-base-300",
        "prose-pre:text-base-content",
        "prose-pre:overflow-auto",
        "prose-code:bg-base-300",
        "prose-code:px-[5.5px]",
        "prose-code:font-normal",
        "prose-code:rounded-[0.3125rem]",
        "prose-code:overflow-auto",
        "prose-a:text-info",
        "prose-table:border-[1px]",
        "prose-table:border-black",
        "prose-tr:border-[1px]",
        "prose-tr:border-black",
        "prose-td:border-[1px]",
        "prose-td:border-black",
        "prose-th:border-[1px]",
        "prose-th:border-black",
        "print:block"
    );
    let markdown = use_markdown();
    let preview_md = convert_to_html(markdown.text.as_str());
    let md_html = Html::from_html_unchecked(preview_md.into());
    html! {
        <div class="max-h-full w-[calc(50%-0.5rem)] border-4 border-skin-buttons rounded shadow-md shadow-lg shadow-skin-buttons overflow-auto px-4 py-0.5">
            <article class={classes} id="preview_box">
                {md_html}
            </article>
        </div>
    }
}
