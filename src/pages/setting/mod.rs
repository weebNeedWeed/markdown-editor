pub mod theme_choosing_form;
pub mod theme_setting;

use theme_setting::ThemeSetting;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;
use crate::hooks::use_applied_theme::use_applied_theme;
use crate::utils::icons::ArrowBarLeft;

#[function_component(SettingPage)]
pub fn setting_page() -> Html {
    let applied_theme = use_applied_theme();
    html! {
        <div class={classes!(applied_theme)}>
            <div class="w-screen h-screen flex flex-col overflow-auto">
                <div class="sticky w-full h-16 shrink-0 shadow-md shadow-slate-400 bg-skin-primary flex px-8 justify-start items-center gap-x-6">
                    <Link<Route> to={Route::Home} >
                        <ArrowBarLeft class="fill-skin-typography w-7 h-7" />
                    </Link<Route>>

                    <h3 class="font-semibold text-xl text-skin-typography">
                        {"Cài đặt"}
                    </h3>
                </div>

                <div class="grow overflow-auto sticky">
                    <div class="px-24 py-6 w-full flex flex-col">
                        <ThemeSetting />
                    </div>
                </div>
            </div>
        </div>
    }
}
