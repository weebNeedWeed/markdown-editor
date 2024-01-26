use super::ui::default_layout::*;
use crate::contexts::theme_context::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeContextProvider>
            <DefaultLayout />
        </ThemeContextProvider>
    }
}
