use crate::contexts::config_context::ConfigContextProvider;
use crate::contexts::markdown_context::MarkdownContextProvider;
use crate::pages::home::HomePage;
use crate::pages::setting::SettingPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/setting")]
    Setting,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Setting => html! { <SettingPage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ConfigContextProvider>
                <MarkdownContextProvider>
                    <Switch<Route> render={switch} />
                </MarkdownContextProvider>
            </ConfigContextProvider>
        </BrowserRouter>
    }
}
