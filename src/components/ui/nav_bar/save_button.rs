use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(SaveButton)]
pub fn save_button() -> Html {
    let displaying = use_state(|| false);
    let onclick = {
        let displaying = displaying.clone();
        Callback::from(move |_| displaying.set(!*displaying))
    };
    html! {
        <div class="relative">
            <Button {onclick}>
                <span class="text-md text-skin-typography font-medium">
                    {"Lưu"}
                </span>

                <span class="border-s-2 h-5 border-skin-typography my-auto"></span>

                <CaretDownFill class="fill-skin-typography" />
            </Button>

            if *displaying {
                <div class="absolute top-full w-full shadow-lg shadow-skin-buttons bg-skin-buttons">
                    <div class="flex flex-col items-center justify-start">
                        <button class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                            <FiletypeMd class="stroke-skin-typography"/>
                            {".md"}
                        </button>
                        <button class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                            <FiletypePdf class="stroke-skin-typography"/>
                            {".pdf"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
