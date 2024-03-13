use super::action_button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(ActionBar)]
pub fn action_bar() -> Html {
    html! {
        <div class="sticky p-2 pb-1 flex justify-start items-center w-full border-b-4 border-skin-buttons">
            <ActionButton
                icon={
                    html!{<Arrow90DegLeft class="fill-skin-buttons w-5 h-5"/>}}
                title={"Undo"}
                onclick={Callback::from(|_| {})}
            />

            <ActionButton
                icon={
                    html!{<Arrow90DegRight class="fill-skin-buttons w-5 h-5"/>}}
                title={"Redo"}
                onclick={Callback::from(|_| {})}
            />

            <ActionButton
                icon={
                    html!{<TypeBold class="fill-skin-buttons w-5 h-5"/>}}
                title={"Bold"}
                onclick={Callback::from(|_| {})}
            />

            <ActionButton
                icon={
                    html!{<TypeItalic class="fill-skin-buttons w-5 h-5"/>}}
                title={"Italic"}
                onclick={Callback::from(|_| {})}
            />
        </div>
    }
}
