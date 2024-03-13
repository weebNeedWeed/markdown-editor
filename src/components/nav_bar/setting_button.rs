use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(SettingButton)]
pub fn setting_button() -> Html {
    html! {
        <Button class="px-3.5">
            <Gear class="stroke-skin-typography"/>
        </Button>
    }
}
