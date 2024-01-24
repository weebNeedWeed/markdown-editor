use yew::prelude::*;

pub enum ThemeAction {}

#[derive(Clone, PartialEq)]
pub struct ThemeState {}

impl ThemeState {
    fn default() -> Self {
        Self {}
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
    let ctx = use_context::<ThemeContext>().expect("ThemeContext must be found");
    ctx
}
