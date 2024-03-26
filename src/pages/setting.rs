use yew::prelude::*;

use crate::contexts::theme_context::{use_theme_context, Theme};

#[function_component(SettingPage)]
pub fn setting_page() -> Html {
    let theme_context = use_theme_context();
    let applied_theme = match theme_context.current {
        Theme::Default => "",
    };

    html! {
        <div class={classes!(applied_theme)}>
            <div class="w-screen h-screen flex flex-col overflow-auto">
                <div class="sticky w-full h-64">

                </div>
            </div>
        </div>
    }
}
