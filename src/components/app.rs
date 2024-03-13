use super::default_layout::*;
use crate::contexts::markdown_context::MarkdownContextProvider;
use crate::contexts::theme_context::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeContextProvider>
            <MarkdownContextProvider>
                <DefaultLayout />
            </MarkdownContextProvider>
        </ThemeContextProvider>
    }
}
