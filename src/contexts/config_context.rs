use std::rc::Rc;

use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Default,
    Black,
    Blue,
}

pub enum ConfigAction {
    ChangeTheme(Theme),
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigState {
    pub current_theme: Theme,
}

impl ConfigState {
    fn load_from_storage() -> Option<Self> {
        if let Ok(config) = LocalStorage::get::<Value>("config") {
            let config: ConfigState = serde_json::from_value(config).unwrap();
            return Some(config);
        };
        None
    }

    fn save_to_storage(&self) {
        LocalStorage::set("config", self).unwrap();
    }
}

impl Default for ConfigState {
    fn default() -> Self {
        Self {
            current_theme: Theme::Default,
        }
    }
}

impl Reducible for ConfigState {
    type Action = ConfigAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ConfigAction::ChangeTheme(theme) => {
                let new_ins = ConfigState {
                    current_theme: theme,
                    ..*self
                };
                self.save_to_storage();
                return Rc::from(new_ins);
            }
        }
    }
}

pub type ConfigContext = UseReducerHandle<ConfigState>;

#[yew_autoprops::autoprops]
#[function_component(ConfigContextProvider)]
pub fn config_context_provider(#[prop_or_default] children: &Html) -> Html {
    let reducer = use_reducer(|| ConfigState::load_from_storage().unwrap_or_default());
    html! {
        <ContextProvider<ConfigContext> context={reducer}>
            {children.clone()}
        </ContextProvider<ConfigContext>>
    }
}

#[hook]
pub fn use_config_context() -> ConfigContext {
    use_context::<ConfigContext>().expect("ConfigContext must be found")
}
