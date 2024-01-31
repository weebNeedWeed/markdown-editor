use super::nav_bar::*;
use crate::components::ui::editor::*;
use crate::contexts::theme_context::{use_theme_context, Theme};
use yew::prelude::*;

#[function_component(DefaultLayout)]
pub fn default_layout() -> Html {
    let theme_context = use_theme_context();
    let applied_theme = match theme_context.current {
        Theme::Default => "",
    };
    html! {
        <div class={classes!(
                applied_theme,
                "fixed", "bg-skin-background",
                "w-screen", "h-screen", "flex", "flex-col")}>
            <header>
                <NavBar />
            </header>

            <div class="w-full p-4 flex flex-row items-stretch grow gap-x-4">
                <Editor />
                <div class="h-full w-[calc(50%-0.5rem)] border-4 border-skin-buttons rounded-xl shadow-md shadow-lg shadow-skin-buttons">
                </div>
            </div>
        </div>
    }
}
