use action_bar::*;
use yew::prelude::*;

pub mod action_bar;

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div class="h-full w-[calc(50%-0.5rem)] border-4 border-skin-buttons rounded-xl shadow-md shadow-lg shadow-skin-buttons">
            <div class="h-full flex flex-col items-start justify-center">
                <ActionBar />
            </div>
        </div>
    }
}
