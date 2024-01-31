use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(ActionBar)]
pub fn action_bar() -> Html {
    html! {
        <div class="flex justify-start items-center w-full">
            <div class="relative group">
                <button
                    class="active:scale-75 transition-all hover:bg-skin-secondary px-2 py-1 rounded-md">
                    <Arrow90DegLeft class="fill-skin-buttons w-5 h-5"/>

                </button>
                <span
                    class="absolute hidden group-hover:block top-full left-1/2 -translate-x-1/2 bg-skin-buttons rounded-md px-2 py-1 text-sm text-white">
                    { "Hello" }
                </span>
            </div>

        </div>
    }
}
