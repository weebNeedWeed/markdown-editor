use action_bar::*;
use yew::prelude::*;

pub mod action_bar;
pub mod action_button;

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div class="h-full w-[calc(50%-0.5rem)] p-4 border-4 border-skin-buttons rounded-xl shadow-md shadow-lg shadow-skin-buttons">
            <div class="h-full flex flex-col items-center justify-start">
                <ActionBar />
            </div>
        </div>
    }
}
