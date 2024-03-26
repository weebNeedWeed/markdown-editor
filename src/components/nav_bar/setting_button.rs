use crate::app::Route;
use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(SettingButton)]
pub fn setting_button() -> Html {
    let navigator = use_navigator().unwrap();
    let handle_click = Callback::from(move |_| navigator.push(&Route::Setting));
    html! {
        <Button onclick={handle_click} class="px-3.5">
            <Gear class="stroke-skin-typography"/>
        </Button>
    }
}
