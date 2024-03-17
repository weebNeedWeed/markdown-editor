use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Default,
}

pub enum ThemeAction {}

#[derive(Clone, PartialEq)]
pub struct ThemeState {
    pub current: Theme,
}

impl Default for ThemeState {
    fn default() -> Self {
        Self {
            current: Theme::Default,
        }
    }
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            _ => self,
        }
    }
}

pub type ThemeContext = UseReducerHandle<ThemeState>;

#[yew_autoprops::autoprops]
#[function_component(ThemeContextProvider)]
pub fn theme_context_provider(#[prop_or_default] children: &Html) -> Html {
    let reducer = use_reducer(ThemeState::default);
    html! {
        <ContextProvider<ThemeContext> context={reducer}>
            {children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub fn use_theme_context() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext must be found")
}
