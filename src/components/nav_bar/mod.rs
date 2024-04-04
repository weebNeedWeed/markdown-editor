use creates::CreateDropdown;
use file_name::FileName;
use saves::SaveDropdown;
use setting_button::SettingButton;
use toggle_button::ToggleButton;
use yew::prelude::*;

pub mod creates;
pub mod file_name;
pub mod saves;
pub mod setting_button;
pub mod toggle_button;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let open = use_state(|| true);
    let handle_toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    html! {
        <nav
            class={classes!(String::from("p-4 pb-0 text-skin-typography transition-all"),
                if *open { "mt-0" } else { "mt-[-100px]" } )}>
            <div class="flex flex-row w-full items-stretch justify-between">
                <div class="flex flex-row items-center gap-x-4">
                    <CreateDropdown open={*open.clone()} />
                    <SaveDropdown />

                    <FileName />
                </div>

                <div class="flex items-stretch gap-x-4">
                    <SettingButton />
                    <ToggleButton onclick={handle_toggle} open={*open}/>
                </div>
            </div>
        </nav>
    }
}
