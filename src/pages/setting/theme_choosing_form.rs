use yew::prelude::*;

use crate::contexts::config_context::{use_config_context, ConfigAction, Theme};

#[function_component(ThemeChoosingForm)]
pub fn theme_choosing_form() -> Html {
    let config_context = use_config_context();
    let value = config_context.current_theme.clone();
    let make_onclick_fn = move |theme: Theme| {
        let config_context = config_context.clone();
        Callback::from(move |_: MouseEvent| {
            config_context.dispatch(ConfigAction::ChangeTheme(theme.clone()))
        })
    };

    html! {
        <form class="grid grid-cols-4 gap-4">
            <label onclick={make_onclick_fn(Theme::Default)} class="flex w-full bg-transparent rounded-md px-2 py-3 border-2 border-[#8D5524] cursor-pointer">
                <input type="radio" name="theme" checked={value.eq(&Theme::Default)} class="accent-[#8D5524] self-start" />
                <span class="w-10 h-10 bg-[#8D5524] ml-3 rounded flex items-center justify-center text-white font-semibold">
                    {"A"}
                </span>
                <span class="w-10 h-10 bg-white border-2 border-[#8D5524] ml-3 rounded flex items-center justify-center text-[#8D5524] font-semibold">
                    {"A"}
                </span>
            </label>


            <label onclick={make_onclick_fn(Theme::Black)} class="flex w-full bg-transparent rounded-md px-2 py-3 border-2 border-black cursor-pointer">
                <input type="radio" name="theme" checked={value.eq(&Theme::Black)} class="accent-black self-start" />
                <span class="w-10 h-10 bg-black ml-3 rounded flex items-center justify-center text-white font-semibold">
                    {"A"}
                </span>
                <span class="w-10 h-10 bg-white border-2 border-black ml-3 rounded flex items-center justify-center text-black font-semibold">
                    {"A"}
                </span>
            </label>

        </form>

    }
}
