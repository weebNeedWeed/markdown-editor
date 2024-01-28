use create_button::*;
use save_button::*;
use setting_button::*;
use toggle_button::*;
use yew::prelude::*;

pub mod create_button;
pub mod save_button;
pub mod setting_button;
pub mod toggle_button;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="p-4 text-skin-typography">
            <div class="flex flex-row w-full items-stretch justify-between">
                <div class="flex flex-row items-center gap-x-4">
                    <CreateButton />
                    <SaveButton />
                </div>

                <div class="flex items-stretch gap-x-4">
                    <SettingButton />
                    <ToggleButton />
                </div>
            </div>
        </nav>
    }
}