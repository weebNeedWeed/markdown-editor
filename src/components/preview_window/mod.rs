use crate::components::raw_html::RawHtml;
use yew::prelude::*;

#[function_component(PreviewWindow)]
pub fn preview_window() -> Html {
    let classes = classes!("prose",);
    html! {
        <div class="max-h-full w-[calc(50%-0.5rem)] border-4 border-skin-buttons rounded shadow-md shadow-lg shadow-skin-buttons overflow-auto p-4">
            <div class={classes}>
                <RawHtml inner_html={""}/>
            </div>
        </div>
    }
}
