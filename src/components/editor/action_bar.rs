use super::action_button::bold::Bold;
use super::action_button::heading::Heading;
use super::action_button::italic::Italic;
use super::action_button::redo::Redo;
use super::action_button::undo::Undo;
use yew::prelude::*;

#[function_component(ActionBar)]
pub fn action_bar() -> Html {
    html! {
        <div class="sticky p-2 pb-1 flex justify-start items-center w-full border-b-4 border-skin-buttons">
            <Undo />
            <Redo />
            <Bold />
            <Italic />
            <Heading />
        </div>
    }
}
