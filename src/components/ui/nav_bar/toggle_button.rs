use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(ToggleButton)]
pub fn toggle_button() -> Html {
    html! {
        <Button class="px-3.5">
            <Close class="stroke-skin-typography"/>
        </Button>
    }
}
