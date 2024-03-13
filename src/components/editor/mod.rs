use action_bar::*;
use edit_section::*;
use yew::prelude::*;

pub mod action_bar;
pub mod action_button;
pub mod edit_section;

#[function_component(Editor)]
pub fn editor() -> Html {
    html! {
        <div class="max-h-full w-[calc(50%-0.5rem)] border-4 border-skin-buttons rounded shadow-md shadow-lg shadow-skin-buttons">
            <div class="h-full flex flex-col items-center justify-start">
                <ActionBar />

                <EditSection />
            </div>
        </div>
    }
}
