use super::action_button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(ActionBar)]
pub fn action_bar() -> Html {
    html! {
        <div class="flex justify-start items-center w-full">
            <ActionButton
                icon={
                    html!{<Arrow90DegLeft class="fill-skin-buttons w-5 h-5"/>}}
                title={"Hello"}
                onclick={Callback::from(|_| {})}
            />
            <ActionButton
                icon={
                    html!{<Arrow90DegRight class="fill-skin-buttons w-5 h-5"/>}}
                title={"Hello"}
                onclick={Callback::from(|_| {})}
            />

        </div>
    }
}
