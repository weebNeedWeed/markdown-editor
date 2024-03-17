use crate::components::button::*;
use crate::utils::icons::*;
use create_button::CreateButton;
use open_recent::OpenRecent;
use upload_button::UploadButton;
use yew::prelude::*;

pub mod create_button;
pub mod open_recent;
pub mod upload_button;

#[yew_autoprops::autoprops]
#[function_component(CreateDropdown)]
pub fn create_dropdown(open: bool) -> Html {
    let displaying = use_state(|| false);
    let handle_open_dropdown = {
        let displaying = displaying.clone();
        Callback::from(move |_| displaying.set(!*displaying))
    };
    {
        let displaying = displaying.clone();
        use_effect_with(open, move |open| {
            if !open {
                displaying.set(false);
            }
        });
    }

    html! {
        <div class="relative">
            <Button onclick={handle_open_dropdown}>
                <span class="text-md text-skin-typography font-medium">
                    {"Tạo mới"}
                </span>

                <span class="border-s-2 h-5 border-skin-typography my-auto"></span>

                <CaretDownFill class="fill-skin-typography" />
            </Button>

            if *displaying {
                <div class="absolute z-10 top-full w-full shadow shadow-skin-buttons bg-skin-buttons">
                    <div class="flex flex-col items-center justify-start">
                        <CreateButton />
                        <UploadButton />

                        <hr class="w-10/12 h-[1px] bg-skin-typography" />

                        <OpenRecent />
                    </div>
                </div>
            }

        </div>
    }
}
