use create_button::*;
use save_button::*;
use setting_button::*;
use toggle_button::*;
use yew::prelude::*;

pub mod create_button;
pub mod save_button;
pub mod setting_button;
pub mod toggle_button;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let open = use_state(|| false);
    let handle_toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    html! {
        <nav
            class={classes!(String::from("p-4 text-skin-typography transition-all"),
                if *open { "mt-0" } else { "mt-[-100px]" } )}>
            <div class="flex flex-row w-full items-stretch justify-between">
                <div class="flex flex-row items-center gap-x-4">
                    <CreateButton />
                    <SaveButton />
                </div>

                <div class="flex items-stretch gap-x-4">
                    <SettingButton />
                    <ToggleButton onclick={handle_toggle} open={*open}/>
                </div>
            </div>
        </nav>
    }
}
