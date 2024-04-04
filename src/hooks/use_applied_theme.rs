use crate::contexts::config_context::use_config_context;
use crate::contexts::config_context::Theme;
use yew::prelude::*;

#[hook]
pub fn use_applied_theme() -> AttrValue {
    let config_context = use_config_context();
    let applied_theme = match config_context.current_theme {
        Theme::Default => "",
        Theme::Black => "theme-black",
        Theme::Blue => "theme-blue",
    };

    AttrValue::from(applied_theme)
}
