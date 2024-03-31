use crate::components::editor::Editor;
use crate::components::nav_bar::NavBar;
use crate::components::preview_window::PreviewWindow;
use crate::hooks::use_applied_theme::use_applied_theme;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let applied_theme = use_applied_theme();
    html! {
        <div class={classes!(
                applied_theme,
                "fixed", "bg-skin-background",
                "w-full", "h-full",
                "w-screen", "h-screen", "flex", "flex-col")}>
            <header>
                <NavBar />
            </header>

            <div class="p-4 flex flex-row max-h-full h-full grow gap-x-4 overflow-auto">
                <Editor />
                <PreviewWindow />
            </div>
        </div>
    }
}
