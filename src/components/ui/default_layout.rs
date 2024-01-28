use super::nav_bar::*;
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
                "fixed", "bg-skin",
                "w-screen", "h-screen")}>
            <header>
                <NavBar />
            </header>
        </div>
    }
}
