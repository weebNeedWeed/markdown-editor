use super::theme_choosing_form::ThemeChoosingForm;
use yew::prelude::*;

#[function_component(ThemeSetting)]
pub fn theme_setting() -> Html {
    html! {
        <div class="flex flex-col gap-3">
            <h4 class="text-black font-normal text-2xl">
                {"Chủ đề giao diện"}
            </h4>
            <ThemeChoosingForm />
        </div>
    }
}
