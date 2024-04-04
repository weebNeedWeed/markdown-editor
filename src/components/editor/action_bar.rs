use super::action_button::bold::Bold;
use super::action_button::heading::Heading;
use super::action_button::image::Image;
use super::action_button::italic::Italic;
use super::action_button::link::Link;
use super::action_button::quote::Quote;
use super::action_button::redo::Redo;
use super::action_button::strikethrough::Strikethrough;
use super::action_button::table::Table;
use super::action_button::undo::Undo;
use yew::prelude::*;

#[function_component(ActionBar)]
pub fn action_bar() -> Html {
    html! {
        <div class="sticky p-2 pb-1 flex justify-start items-center w-full border-b-4 border-skin-buttons">
            <Undo />
            <Redo />

            <span class="h-5/6 border-l-[1px] border-skin-primary mx-3 opacity-90"></span>

            <Heading />
            <Bold />
            <Italic />
            <Strikethrough />

            <span class="h-5/6 border-l-[1px] border-skin-primary mx-3 opacity-90"></span>

            <Quote />
            <Link />
            <Image />
            <Table />
        </div>
    }
}
