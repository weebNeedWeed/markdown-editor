use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct ToggleButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub open: bool,
}

#[function_component(ToggleButton)]
pub fn toggle_button(ToggleButtonProps { onclick, open }: &ToggleButtonProps) -> Html {
    html! {
        <Button
            class={
                classes!("px-3.5",if !*open {String::from("fixed top-4 py-3.5")} else {String::default()})}
            onclick={onclick.clone()}>
            if *open {
                <Close class="stroke-skin-typography"/>
            } else {
                <HamburgerMenu class="stroke-skin-typography"/>
            }
        </Button>
    }
}
