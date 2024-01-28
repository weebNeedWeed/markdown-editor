use super::ui::default_layout::*;
use crate::contexts::file_context::*;
use crate::contexts::theme_context::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeContextProvider>
            <FileContextProvider>
                <DefaultLayout />
            </FileContextProvider>
        </ThemeContextProvider>
    }
}
